use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgba(0.15, 0.15, 0.15, 0.7);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgba(0.50, 0.50, 0.50, 0.7);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.7);

pub fn get_main_menu_style() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        row_gap: Val::Px(8.0),
        ..Style::DEFAULT
    }
}

pub fn get_title_style() -> Style {
    Style {
        flex_direction: FlexDirection::Row,
        align_content: AlignContent::Center,
        align_self: AlignSelf::Center,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Px(60.0),
        margin: UiRect {
            left: Val::Px(20.),
            bottom: Val::Px(100.0),
            ..default()
        },
        ..Style::DEFAULT
    }
}

pub fn get_normal_button_style() -> Style {
    Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(150.0),
        height: Val::Px(70.0),
        margin: UiRect::vertical(Val::Px(20.)),
        ..Style::DEFAULT
    }
}

// Text Styles
pub fn get_menu_button_text(text: &str, _asset_server: &Res<AssetServer>) -> Text {
    Text {
        sections: vec![TextSection::new(
            text,
            TextStyle {
                font_size: 15.0,
                color: Color::WHITE,
                ..default()
            },
        )],
        justify: JustifyText::Center,
        ..default()
    }
}
