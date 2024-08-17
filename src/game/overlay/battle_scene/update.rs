use bevy::prelude::*;

use crate::game::{event::AttackEvent, TurnFlag};

use super::{HealthBar, HeroHealthBar, LvUpText, MonsterHealthBar};

pub fn update_hero_health_bar(
    mut event: EventReader<AttackEvent>,
    mut bar_query: Query<&mut HealthBar, With<HeroHealthBar>>,
) {
    for ev in event.read() {
        if !ev.record.is_player_turn {
            info!("Update Hero Health Bar: {}", ev.record.damage_out.get());
            for mut bar in &mut bar_query {
                bar.remain -= ev.record.damage_out.get();
            }
        }
    }
}

pub fn damping_hero_health_bar(
    mut bar_query: Query<(&mut Style, &mut BackgroundColor, &HealthBar), With<HeroHealthBar>>,
    time: Res<Time>,
) {
    for (mut bar_style, _background_color, bar) in &mut bar_query {
        let Val::Percent(current_width) = bar_style.width else {
            continue;
        };

        if current_width > bar.get_percentage() {
            bar_style.width = Val::Percent(
                bar.get_percentage()
                    .max(current_width - 100. * time.delta_seconds()),
            );
        }
    }
}

pub fn update_monster_health_bar(
    mut event: EventReader<AttackEvent>,
    mut bar_query: Query<&mut HealthBar, With<MonsterHealthBar>>,
) {
    for ev in event.read() {
        if ev.record.is_player_turn {
            for mut bar in &mut bar_query {
                bar.remain -= ev.record.damage_out.get();
                info!("Update Monster Health Bar: {}", bar.get_percentage());
            }
        }
    }
}

pub fn damping_monster_health_bar(
    mut bar_query: Query<(&mut Style, &mut BackgroundColor, &HealthBar), With<MonsterHealthBar>>,
    time: Res<Time>,
) {
    for (mut bar_style, _background_color, bar) in &mut bar_query {
        let Val::Percent(current_width) = bar_style.width else {
            continue;
        };

        if current_width > bar.get_percentage() {
            // info!(
            //     "new monster health percentage: {}",
            //     current_width - 100. * time.delta_seconds()
            // );
            bar_style.width = Val::Percent(
                bar.get_percentage()
                    .max(current_width - 100. * time.delta_seconds()),
            );
        }
    }
}

pub fn update_lv_up_text_position(
    mut lv_up_query: Query<&mut Style, With<LvUpText>>,
    time: Res<Time>,
) {
    for mut style in lv_up_query.iter_mut() {
        if let Val::Px(top) = style.top {
            style.top = Val::Px(top - 10. * time.delta_seconds());
        } else {
            style.top = Val::Px(0.0);
        }
    }
}

pub fn update_flag_on_finish(
    mut battle_query: Query<&mut TurnFlag>,
    hero_bar_query: Query<(&mut Style, &HealthBar), With<HeroHealthBar>>,
    monster_bar_query: Query<
        (&mut Style, &HealthBar),
        (With<MonsterHealthBar>, Without<HeroHealthBar>),
    >,
) {
    let Ok(mut turn_flag) = battle_query.get_single_mut() else {
        warn!("No turn flag found");
        return;
    };

    let (Ok((hero_bar_style, hero_bar)), Ok((monster_bar_style, monster_bar))) =
        (hero_bar_query.get_single(), monster_bar_query.get_single())
    else {
        warn!("No health bars found");
        return;
    };

    let (Val::Percent(hero_width), Val::Percent(monster_width)) =
        (hero_bar_style.width, monster_bar_style.width)
    else {
        return;
    };

    let hero_diff = (hero_width - hero_bar.get_percentage()).abs();
    let monster_diff = (monster_width - monster_bar.get_percentage()).abs();

    if hero_diff < 0.5 && monster_diff < 0.5 {
        turn_flag.is_animation_ready = true;
    } else {
        turn_flag.is_animation_ready = false;
    }
}
