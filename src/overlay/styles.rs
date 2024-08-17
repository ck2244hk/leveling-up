use bevy::{
    ui::{AlignItems, JustifyContent, Style, UiRect, Val},
    utils::default,
};

pub fn get_buy_button_style() -> Style {
    Style {
        padding: UiRect {
            left: Val::Px(10.),
            right: Val::Px(10.),
            top: Val::ZERO,
            bottom: Val::ZERO,
        },
        width: Val::Percent(30.),
        min_width: Val::Px(70.),
        max_width: Val::Px(150.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        height: Val::Px(40.0),
        ..default()
    }
}
