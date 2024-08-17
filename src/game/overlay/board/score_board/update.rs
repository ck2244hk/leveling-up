use crate::game::{
    character::component::{BaseStates, Hero},
    overlay::board::components::{ExperienceText, LevelText, MoneyText},
    Money, Player,
};
use bevy::prelude::*;

pub fn update_lv_text(
    mut text_query: Query<(&mut Text, &mut LevelText)>,
    player_query: Query<&BaseStates, With<Hero>>,
    time: Res<Time>,
) {
    if let Ok((mut text, mut level)) = text_query.get_single_mut() {
        if let Ok(stat) = player_query.get_single() {
            if level.0 < stat.lv_f32() {
                level.0 = stat
                    .lv_f32()
                    .min(level.0 as f32 + stat.lv_f32() * time.delta_seconds() / 3.);
            }

            text.sections[0].value = format!("{:.0}Lv", level.0);
        }
    }
}

pub fn update_money_text(
    mut text_query: Query<(&mut Text, &mut MoneyText)>,
    player_query: Query<&Money, With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut text, mut money)) = text_query.get_single_mut() {
        if let Ok(stat) = player_query.get_single() {
            if money.0 < stat.get_f32() {
                money.0 = stat
                    .get_f32()
                    .min(money.0 as f32 + stat.get() as f32 * time.delta_seconds() / 3.);
            }

            text.sections[0].value = format!("Money: {:.0}", money.0);
        }
    }
}

pub fn update_exp_text(
    mut text_query: Query<(&mut Text, &mut ExperienceText)>,
    player_query: Query<&BaseStates, With<Hero>>,
    time: Res<Time>,
) {
    if let Ok((mut text, mut exp_text)) = text_query.get_single_mut() {
        if let Ok(state) = player_query.get_single() {
            let total = state.total_exp() as f32;
            if exp_text.0 < total {
                exp_text.0 = total.min(exp_text.0 + (total) * time.delta_seconds() / 3.);
            }

            text.sections[0].value = format!("Exp: {:.0}", exp_text.0);
        }
    }
}
