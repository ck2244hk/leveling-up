use bevy::prelude::*;

use crate::game::battle_scene::component::LvUpQueue;
use crate::game::character::component::*;


pub fn update_player_level(
    mut player_query: Query<
        (&mut Level, &mut Experience, &mut StatePoint),
        (With<Hero>, Changed<Experience>),
    >,
    mut hero_lv_up_text_queue: Query<&mut LvUpQueue>,
) {
    if let Ok((mut player_level, mut player_experience, mut state_point)) =
        player_query.get_single_mut()
    {
        if let Ok(mut lv_up_queue) = hero_lv_up_text_queue.get_single_mut() {
            while player_experience.0 >= player_level.total_exp_req_for_next_lv() {
                player_level.upgrade();
                state_point.plus_one();
                player_experience.0 -= player_level.total_exp_req_for_next_lv();
                println!("level up to {}", player_level.get());
                lv_up_queue.0 += 1;
            }
        }
    }
}
