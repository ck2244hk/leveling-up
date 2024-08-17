//! A Bevy plugin the provides a simple single-line text input widget.

mod auto_system;
pub mod combat_system;
pub mod component;
mod talk_system;

use std::collections::VecDeque;

use bevy::{
    asset::load_internal_binary_asset, ecs::system::SystemParam, prelude::*, text::BreakLineOn,
};
use component::AutoTerminalText;

use crate::{
    state::{OverlayCombatState, OverlayStatusBoardState, TerminalState},
    helper::string_to_char,
};

use auto_system::*;
use combat_system::*;
pub use talk_system::*;

#[derive(Event)]
pub struct WriteTerminalEvent(String);

impl WriteTerminalEvent {
    pub fn new(text: String) -> WriteTerminalEvent {
        WriteTerminalEvent(text)
    }
}

/// A `Plugin` providing the systems and assets required to make a [`TerminalBundle`] work.
pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        // This is a special font with a zero-width `|` glyph.
        load_internal_binary_asset!(
            app,
            CURSOR_HANDLE,
            "../../../../assets/fonts/Cursor.ttf",
            |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
        );

        app.add_event::<WriteTerminalEvent>()
            .add_systems(
                Update,
                (
                    next_sentance_in_auto,
                    check_auto_text,
                    handle_sentance_pause_timer,
                )
                    .run_if(in_state(TerminalState::Auto))
                    .run_if(in_state(OverlayStatusBoardState::Opened)),
            )
            .add_systems(
                Update,
                (
                    handle_click_in_combat,
                    set_ready_for_next_turn,
                    next_sentance_combat,
                )
                    .run_if(in_state(TerminalState::Combating))
                    .run_if(in_state(OverlayCombatState::Opened)),
            )
            // .add_systems(OnEnter(TerminalState::Talking), clear_queue_on_enter)
            .add_systems(
                Update,
                (
                    handle_click_in_talking,
                    check_talk_state,
                    next_sentance_in_talking,
                )
                    .run_if(in_state(TerminalState::Talking))
                    .run_if(in_state(OverlayCombatState::Opened)),
            )
            .add_systems(
                Update,
                (
                    create,
                    show_hide_cursor,
                    update_style,
                    event_to_queue,
                    push_queue_to_inner.before(event_to_queue),
                    // handle_battle_end.after(event_to_queue),
                    handle_vertical_overflow,
                ),
            )
            .register_type::<AutoTerminalText>();
    }
}

const CURSOR_HANDLE: Handle<Font> = Handle::weak_from_u128(10482756907980398621);

/// A bundle providing the additional components required for a text input.
///
/// Add this to a `NodeBundle`.
///
/// Examples:
/// ```rust
/// # use bevy::prelude::*;
// / use bevy_simple_text_input::TerminalBundle;
// / fn setup(mut commands: Commands) {
// /     commands.spawn((NodeBundle::default(), TerminalBundle::default()));
// / }
// / ```
#[derive(Bundle)]
pub struct TerminalBundle {
    text_style: TerminalTextStyle,
    cursor_timer: TerminalCursorTimer,
    text_input: Terminal,
    interaction: Interaction,
    queue: TerminalQueue,
    queue_timer: TerminalQueueTimer,
}

impl TerminalBundle {
    /// Creates a new `TerminalBundle` with the specified `TextStyle`.
    pub fn new(font_size: f32) -> Self {
        Self {
            text_style: TerminalTextStyle(TextStyle {
                font_size,
                color: Color::rgb(0.3, 0.3, 0.3),
                ..default()
            }),
            cursor_timer: TerminalCursorTimer::default(),
            text_input: Terminal,
            interaction: Interaction::default(),
            queue: TerminalQueue::default(),
            queue_timer: TerminalQueueTimer::default(),
        }
    }

    pub fn new_with_auto(font_size: f32) -> (Self, TerminalSentancePauseTimer, AutoTerminalText) {
        (
            Self {
                text_style: TerminalTextStyle(TextStyle {
                    font_size,
                    color: Color::rgb(0.3, 0.3, 0.3),
                    ..default()
                }),
                cursor_timer: TerminalCursorTimer::default(),
                text_input: Terminal,
                interaction: Interaction::default(),
                queue: TerminalQueue::default(),
                queue_timer: TerminalQueueTimer::new(0.05),
            },
            TerminalSentancePauseTimer::default(),
            AutoTerminalText::default_text(),
        )
    }
}

#[derive(Component)]
pub struct TerminalSentancePauseTimer(pub Timer);

impl Default for TerminalSentancePauseTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(2., TimerMode::Once);
        timer.pause();
        Self(timer)
    }
}

/// The `TextStyle` that will be used when creating the text input's inner `TextBundle`.
#[derive(Component, Default)]
pub struct TerminalTextStyle(pub TextStyle);

/// The timer controlling the blinking cursor. The cursor is toggled when the timer is finished.
#[derive(Component)]
pub struct TerminalCursorTimer(pub Timer);
impl Default for TerminalCursorTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

/// The timer controlling the speed of writing.
#[derive(Component)]
pub struct TerminalQueueTimer {
    pub timer: Timer,
    pub duration: f32,
}

impl Default for TerminalQueueTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.01, TimerMode::Once),
            duration: 0.01,
        }
    }
}

impl TerminalQueueTimer {
    fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
            duration,
        }
    }

    fn reset(&mut self) {
        self.timer = Timer::from_seconds(self.duration, TimerMode::Once)
    }
}

/// A marker component for the text input.
#[derive(Component)]
pub struct Terminal;

#[derive(Component)]
struct TerminalInner;

#[derive(Component)]
pub struct TerminalQueue(VecDeque<VecDeque<char>>);

impl Default for TerminalQueue {
    fn default() -> Self {
        TerminalQueue(VecDeque::from(VecDeque::new()))
        // TerminalQueue(VecDeque::from([string_to_char("{".to_string())]))
    }
}
impl TerminalQueue {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// A convenience parameter for dealing with a `Terminal`'s inner `Text` entity.
#[derive(SystemParam)]
pub struct InnerText<'w, 's> {
    text_query: Query<'w, 's, &'static mut Text, With<TerminalInner>>,
    children_query: Query<'w, 's, &'static Children>,
}
impl<'w, 's> InnerText<'w, 's> {
    fn get_mut(&mut self, entity: Entity) -> Option<Mut<'_, Text>> {
        self.children_query
            .iter_descendants(entity)
            .find(|descendant_entity| self.text_query.get(*descendant_entity).is_ok())
            .and_then(|text_entity| self.text_query.get_mut(text_entity).ok())
    }
}

fn create(mut commands: Commands, query: Query<(Entity, &TerminalTextStyle), Added<Terminal>>) {
    for (entity, style) in &query {
        let text = commands
            .spawn((
                TextBundle {
                    style: Style {
                        max_height: Val::Vh(50.),
                        margin: UiRect::bottom(Val::Px(12.)),
                        ..default()
                    },
                    text: get_empty_terminal(style),
                    ..default()
                },
                TerminalInner,
            ))
            .id();

        let overflow_container = commands
            .spawn(NodeBundle {
                style: Style {
                    overflow: Overflow::clip(),
                    justify_content: JustifyContent::End,
                    max_width: Val::Percent(100.),
                    max_height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            })
            .id();

        commands.entity(overflow_container).add_child(text);
        commands.entity(entity).add_child(overflow_container);
    }
}

fn show_hide_cursor(
    mut input_query: Query<(Entity, &TerminalTextStyle, &mut TerminalCursorTimer)>,
    mut inner_text: InnerText,
    time: Res<Time>,
) {
    for (entity, style, mut timer) in &mut input_query {
        if timer.0.tick(time.delta()).just_finished() {
            if let Some(mut text) = inner_text.get_mut(entity) {
                if text
                    .sections
                    .last_mut()
                    .expect("Cursor Disappered")
                    .style
                    .color
                    != Color::NONE
                {
                    text.sections
                        .last_mut()
                        .expect("Cursor Disappered")
                        .style
                        .color = Color::NONE;
                } else {
                    text.sections
                        .last_mut()
                        .expect("Cursor Disappered")
                        .style
                        .color = style.0.color;
                }
            }
        }
    }
}

fn update_style(
    mut input_query: Query<(Entity, &TerminalTextStyle), Changed<TerminalTextStyle>>,
    mut inner_text: InnerText,
) {
    for (entity, style) in &mut input_query {
        let Some(mut text) = inner_text.get_mut(entity) else {
            continue;
        };
        let text_len = text.sections.len();
        for i in 0..text_len - 1 {
            text.sections[i].style = style.0.clone();
        }

        text.sections[text_len - 1].style = TextStyle {
            font: CURSOR_HANDLE,
            ..style.0.clone()
        };
    }
}

fn handle_vertical_overflow(
    mut text_input_query: Query<Entity, With<Terminal>>,
    mut inner_text: InnerText,
) {
    for input_entity in text_input_query.iter_mut() {
        let Some(mut text) = inner_text.get_mut(input_entity) else {
            continue;
        };

        if text.sections.len() >= 5 {
            text.sections.drain(0..1);
        }
    }
}

fn push_queue_to_inner(
    mut text_input_query: Query<
        (
            Entity,
            &mut TerminalQueue,
            &mut TerminalQueueTimer,
            &TerminalTextStyle,
        ),
        With<Terminal>,
    >,
    mut inner_text: InnerText,
    time: Res<Time>,
) {
    for (input_entity, mut queue, mut queue_timer, style) in text_input_query.iter_mut() {
        if queue_timer.timer.tick(time.delta()).finished() {
            let Some(first_item) = queue.0.front_mut() else {
                continue;
            };

            let Some(ch) = first_item.pop_front() else {
                continue;
            };

            let Some(mut text) = inner_text.get_mut(input_entity) else {
                continue;
            };

            if ch == 0xA as char {
                text.sections.last_mut().expect("Cursor Disappeared").value = "".to_string();
                text.sections.last_mut().expect("Cursor Disappeared").style = style.0.clone();
                text.sections.push(TextSection {
                    value: "}".to_string(),
                    style: TextStyle {
                        font: CURSOR_HANDLE,
                        ..style.0.clone()
                    },
                })
            }

            let len: usize = text.sections.len();

            text.sections[len - 2].value.push(ch);
            // info!("ch: {}, ", ch);
            if first_item
                .front()
                .is_some_and(|next_ch| *next_ch == 0xA as char)
            {
                queue_timer.timer = Timer::from_seconds(0.1, TimerMode::Once);
            } else {
                queue_timer.reset();
            }
        }
    }
}

fn event_to_queue(
    mut write_event: EventReader<WriteTerminalEvent>,
    mut text_input_query: Query<&mut TerminalQueue, With<Terminal>>,
) {
    for ev in write_event.read() {
        for mut queue in text_input_query.iter_mut() {
            queue.0.push_back(string_to_char(ev.0.to_string()));
            info!("writing terminal event: {}, ", ev.0.to_string());
            // is_end_of_action.set_if_neq(IsEndofAction(false));
        }
    }
}

// internal functions
fn clear_queue_container(
    text: &mut Text,
    container: &mut VecDeque<char>,
    style: &TerminalTextStyle,
) {
    while let Some(front) = container.pop_front() {
        info!("clearing container: {}, ", front);
        if front == 0xA as char {
            text.sections.last_mut().expect("Cursor Disappeared").value = "".to_string();
            text.sections.last_mut().expect("Cursor Disappeared").style = style.0.clone();
            text.sections.push(TextSection {
                value: "}".to_string(),
                style: TextStyle {
                    font: CURSOR_HANDLE,
                    ..style.0.clone()
                },
            })
        }

        let len: usize = text.sections.len();
        text.sections[len - 2].value.push(front);
    }
}

fn get_empty_terminal(style: &TerminalTextStyle) -> Text {
    Text {
        linebreak_behavior: BreakLineOn::WordBoundary,
        sections: vec![
            // Pre-cursor
            TextSection {
                value: "".to_string(),
                style: style.0.clone(),
            },
            // cursor
            TextSection {
                value: "}".to_string(),
                style: TextStyle {
                    font: CURSOR_HANDLE,
                    ..style.0.clone()
                },
            },
        ],
        justify: JustifyText::Left,
    }
}
