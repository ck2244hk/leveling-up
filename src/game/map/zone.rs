use bevy::{prelude::*, utils::HashMap};

use crate::{
    game::{Player, PlayerEnv},
    state::{OverlayCombatState, Screen, SimulationState},
};

use super::preload::{CollisionDetection, Weather, Zone};

pub struct ZonePlugin;

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(OverlayCombatState::Closed), update_weather)
            .add_systems(
                Update,
                (sync_weather, detect_in_zone)
                    .run_if(in_state(SimulationState::Running))
                    .run_if(in_state(Screen::Playing)),
            );
    }
}

pub fn sync_weather(mut weather_query: Query<(&mut Weather, &Zone), Changed<Weather>>) {
    let mut weather_map: HashMap<u32, Weather> = HashMap::new();
    for (mut weather, zone) in weather_query.iter_mut() {
        if let Some(existing) = weather_map.get(&zone.id) {
            *weather = existing.clone();
        } else {
            weather_map.insert(zone.id, weather.clone());
        }
    }
}

pub fn update_weather(
    mut weather_query: Query<&mut Weather>,
    mut zone_query: Query<&mut CollisionDetection, (Changed<CollisionDetection>, With<Zone>)>,
) {
    info!("Weather Changed");
    for mut weather in weather_query.iter_mut() {
        *weather = Weather::randomize();
    }

    // for mut zone in zone_query.iter_mut() {
    //     zone.set_changed();
    // }
}

pub fn detect_in_zone(
    mut player_query: Query<&mut PlayerEnv, With<Player>>,
    zone_query: Query<(&mut CollisionDetection, &Weather), (With<CollisionDetection>, With<Zone>)>,
) {
    // Avoid unnecessary work
    if zone_query.is_empty() {
        return;
    }
    let Ok(mut player_env) = player_query.get_single_mut() else {
        return;
    };

    let mut is_any = false;

    for (zone, weather) in zone_query.iter() {
        if zone.0 {
            is_any = true;
            player_env.set_if_neq(PlayerEnv(Some(weather.clone())));
            // info!("Player Env: {:?}", weather.clone());
        }
    }

    if !is_any {
        player_env.set_if_neq(PlayerEnv(None));
        // info!("Player Env Clear");
    }
}
