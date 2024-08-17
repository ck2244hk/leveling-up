use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::{
    dynamics::RigidBody,
    geometry::{Collider, Friction},
    pipeline::CollisionEvent,
};
use zone::ZonePlugin;

use crate::preload::map::{MapAsset, MapHandles};

use crate::state::Screen;
pub mod component;
pub mod field;
pub mod preload;
pub mod zone;

use field::FieldPlugin;

use super::Player;
use preload::{CollisionDetection, LdtkPreloadPlugin, Location, Map, Port, Wall};

#[derive(Event)]
pub struct SpawnMapEvent {}

#[derive(Event)]
pub struct ChangeMapEvent {
    pub location: Location,
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnMapEvent>()
            .add_event::<ChangeMapEvent>()
            .add_plugins((FieldPlugin, LdtkPreloadPlugin, ZonePlugin))
            .insert_resource(LevelSelection::index(0))
            .add_systems(OnEnter(Screen::Playing), spawn_map)
            .add_systems(
                Update,
                (
                    spawn_wall_collision,
                    spawn_map_on_touch,
                    detect_player_collidable,
                ),
            )
            .add_systems(OnExit(Screen::GameOver), despawn_ldtk);
    }
}

pub fn spawn_map(
    mut commands: Commands,
    map: Query<&Map, With<Map>>,
    ldtk_map: Res<MapHandles>,
    mut next_map: ResMut<LevelSelection>,
) {
    // commands.spawn(
    //     SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgb(0.25, 0.25, 0.75),
    //         custom_size: Some(Vec2::new(500.0, 500.0)),
    //         ..default()
    //     },
    //     transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
    //     ..default()
    // }
    // );
    // for _ in spawn_map_event_reader.read() {
    if map.is_empty() {
        next_map.set_if_neq(LevelSelection::index(0));
        commands.spawn((
            Name::new("LTDK"),
            Map,
            LdtkWorldBundle {
                ldtk_handle: ldtk_map[&MapAsset::Forbidden].clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                level_set: LevelSet::from_iids([
                    "24c90fe0-d7b0-11ee-bf4e-a90834c55fc0".to_string(),
                    "c4f585e0-d7b0-11ee-88a4-6b9f62256f9a".to_string(),
                ]),
                ..Default::default()
            },
        ));
    } else {
        info!("With New Map go to Level 0");
        next_map.set_if_neq(LevelSelection::index(0));
    }

    // }
}

pub fn spawn_map_on_touch(
    port_query: Query<(&Port, &CollisionDetection), Changed<CollisionDetection>>,
    mut next_map: ResMut<LevelSelection>,
    mut change_map_event: EventWriter<ChangeMapEvent>,
) {
    for (port, coliision) in port_query.iter() {
        if coliision.0 {
            info!(
                "Collide With Port to Map {}, To location: {:?}",
                port.map,
                port.location.clone()
            );
            next_map.set_if_neq(LevelSelection::index(port.map));
            change_map_event.send(ChangeMapEvent {
                location: port.location.clone(),
            });
        }
    }
}

/// Spawns heron collisions for the walls of a level
///
/// You could just insert a ColliderBundle in to the WallBundle,
/// but this spawns a different collider for EVERY wall tile.
/// This approach leads to bad performance.
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
///
/// The algorithm used here is a nice compromise between simplicity, speed,
/// and a small number of rectangle colliders.
/// In basic terms, it will:
/// 1. consider where the walls are
/// 2. combine wall tiles into flat "plates" in each individual row
/// 3. combine the plates into rectangles across multiple rows wherever possible
/// 4. spawn colliders for each rectangle
pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.iter().for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.iter().for_each(|(level_entity, level_iid)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .as_standalone()
                    .get_loaded_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level.layer_instances()[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut wall_rects: Vec<Rect> = Vec::new();

                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for prev_plate in &prev_row {
                        if !current_row.contains(prev_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                wall_rects.push(rect);
                            }
                        }
                    }
                    for plate in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|e| e.top += 1)
                            .or_insert(Rect {
                                bottom: y as i32,
                                top: y as i32,
                                left: plate.left,
                                right: plate.right,
                            });
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        level
                            .spawn_empty()
                            .insert(Collider::cuboid(
                                (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                            ))
                            .insert(RigidBody::Fixed)
                            .insert(Friction::new(1.0))
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
}

pub fn despawn_ldtk(ldtk_query: Query<Entity, With<Map>>, mut commands: Commands) {
    for entity in ldtk_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn detect_player_collidable(
    mut player: Query<&Player>,
    mut danger_zones: Query<&mut CollisionDetection, Without<Player>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                info!(
                    "Collision Start: collider _a: {:?}, collider_b: {:?}",
                    collider_a, collider_b
                );
                if let (Ok(_), Ok(mut collision_detection)) = (
                    player.get_mut(*collider_a),
                    danger_zones.get_mut(*collider_b),
                ) {
                    collision_detection.set_if_neq(CollisionDetection(true));
                }
                if let (Ok(_), Ok(mut collision_detection)) = (
                    player.get_mut(*collider_b),
                    danger_zones.get_mut(*collider_a),
                ) {
                    collision_detection.set_if_neq(CollisionDetection(true));
                };
            }
            CollisionEvent::Stopped(collider_a, collider_b, _) => {
                info!(
                    "Collision End: collider _a: {:?}, collider_b: {:?}",
                    collider_a, collider_b
                );
                if let (Ok(_), Ok(mut collision_detection)) = (
                    player.get_mut(*collider_a),
                    danger_zones.get_mut(*collider_b),
                ) {
                    collision_detection.set_if_neq(CollisionDetection(false));
                }
                if let (Ok(_), Ok(mut collision_detection)) = (
                    player.get_mut(*collider_b),
                    danger_zones.get_mut(*collider_a),
                ) {
                    collision_detection.set_if_neq(CollisionDetection(false));
                };
            }
        }
    }
}
