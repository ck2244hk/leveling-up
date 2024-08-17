use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.50, 0.50, 0.50);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

pub fn get_main_menu_style() -> Style {
    Style {
        flex_direction: FlexDirection::Row,
        width: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,

        padding: UiRect {
            left: Val::Px(10.),
            right: Val::Px(10.),
            top: Val::Px(50.),
            bottom: Val::Px(2.),
        },
        ..Style::DEFAULT
    }
}

pub fn get_normal_button_style() -> Style {
    Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(100.0),
        height: Val::Px(40.0),
        ..Style::DEFAULT
    }
}
