use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

pub fn get_normal_button_style() -> Style {
    Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(150.0),
        height: Val::Px(60.0),
        ..Style::DEFAULT
    }
}

// pub fn get_stat_row_style() -> Style {
//     Style {
//         flex_direction: FlexDirection::Row,
//         width: Val::Percent(100.),
//         align_items: AlignItems::Center,
//         align_content: AlignContent::Center,
//         justify_content: JustifyContent::SpaceBetween,

//         ..Style::DEFAULT
//     }
// }

// pub fn get_stat_text_style() -> Style {
//     Style {
//         width: Val::Px(100.),
//         max_width: Val::Percent(50.),
//         ..Style::DEFAULT
//     }
// }

// pub fn get_state_button_style() -> Style {
//     Style {
//         justify_content: JustifyContent::Center,
//         align_items: AlignItems::Center,
//         width: Val::Px(45.0),
//         height: Val::Px(30.0),
//         ..Style::DEFAULT
//     }
// }
