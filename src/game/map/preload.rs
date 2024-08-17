use bevy::prelude::*;
use bevy_ecs_ldtk::{
    app::{LdtkEntityAppExt, LdtkIntCellAppExt},
    ldtk::ldtk_fields::LdtkFields,
    EntityInstance, GridCoords, LdtkEntity, LdtkIntCell,
};
use bevy_rapier2d::{
    dynamics::LockedAxes,
    geometry::{ActiveEvents, Collider, Sensor},
};
use rand::seq::SliceRandom;

pub struct LdtkPreloadPlugin;

impl Plugin for LdtkPreloadPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell_for_layer::<WallBundle>("Wall", 1)
            .register_ldtk_entity::<FieldBundle>("Field")
            .register_ldtk_entity::<PortBundle>("Port")
            .register_ldtk_entity::<SpawnLocationBundle>("Player")
            .register_ldtk_entity::<ZoneBundle>("Zone")
            .register_type::<Field>()
            .register_type::<CollisionDetection>()
            .register_type::<Weather>();
    }
}

#[derive(Component, Clone, Default)]
pub struct Zone {
    pub id: u32,
}

impl From<&EntityInstance> for Zone {
    fn from(entity_instance: &EntityInstance) -> Zone {
        Self {
            id: entity_instance
                .get_int_field("Id")
                .expect("expected entity to have non-nullable name string field")
                .clone() as u32,
        }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct ZoneBundle {
    #[with(name_from_zone)]
    name: Name,
    #[from_entity_instance]
    pub zone: Zone,
    pub weather: Weather,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    pub collision_detection: CollisionDetection,
}

#[derive(Component, Clone)]
pub struct ShapeWrapper(pub Vec2);

impl Default for ShapeWrapper {
    fn default() -> Self {
        ShapeWrapper(Vec2::ZERO)
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}

impl From<&EntityInstance> for SensorBundle {
    fn from(_: &EntityInstance) -> SensorBundle {
        // ladder
        // info!(
        //     "Zone Width: {}, Zone Height: {}",
        //     entity_instance.width, entity_instance.height
        // );
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        // let width = entity_instance.width as f32;
        // let height = entity_instance.height as f32;

        Self {
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
            collider: Collider::cuboid(8., 8.),
            rotation_constraints,
        }
    }
}

impl From<&EntityInstance> for ShapeWrapper {
    fn from(entity_instance: &EntityInstance) -> ShapeWrapper {
        // // let width = (entity_instance.width as f32) / 512. + 7.;
        // let width =  as f32 / RAPIER_PIXELS_PER_METER;
        // let height =  as f32 / RAPIER_PIXELS_PER_METER;

        // let shape = Rectangle::new(16., 16.).mesh();

        // info!(
        //     "Field {}: Width: {}, Height: {}, After Width: {}, After Height: {}",
        //     entity_instance
        //         .get_int_field("Level")
        //         .expect("expected entity to have non-nullable name string field")
        //         .clone(),
        //     entity_instance.width,
        //     entity_instance.height,
        //     width,
        //     height
        // );

        Self(Vec2::new(
            entity_instance.width as f32,
            entity_instance.height as f32,
        ))
    }
}

#[derive(Component, Default, Reflect, PartialEq, Eq)]
pub struct CollisionDetection(pub bool);

#[derive(Component, Default, Reflect)]
pub struct Field {
    pub level: u32,
    pub monster_id: Vec<i32>,
    pub zone_id: u32,
}

impl Field {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        let monster_ids: Vec<i32> = (*entity_instance
            .get_maybe_ints_field("MonsterID")
            .expect("expected entity to have non-nullable monster int field")
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<i32>>())
        .to_vec();

        Self {
            level: *entity_instance
                .get_int_field("Level")
                .expect("expected entity to have non-nullable level int field")
                as u32,
            monster_id: monster_ids,
            zone_id: *entity_instance
                .get_int_field("ZoneId")
                .expect("expected entity to have non-nullable level int field")
                as u32,
        }
    }

    pub fn lv(&self) -> u32 {
        self.level
    }
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct FieldBundle {
    #[with(name_from_field)]
    name: Name,
    #[with(Field::from_field)]
    area: Field,
    // #[grid_coords]
    // grid_coords: GridCoords,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    pub collision_detection: CollisionDetection,
    #[from_entity_instance]
    pub shape: ShapeWrapper,
}

fn name_from_field(entity_instance: &EntityInstance) -> Name {
    Name::new(format!(
        "Field {}",
        entity_instance
            .get_int_field("Level")
            .expect("expected entity to have non-nullable name string field")
            .clone()
    ))
}

fn name_from_zone(entity_instance: &EntityInstance) -> Name {
    Name::new(format!(
        "Zone {}",
        entity_instance
            .get_int_field("Id")
            .expect("expected entity to have non-nullable name string field")
            .clone()
    ))
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PortBundle {
    #[with(Port::from_field)]
    port: Port,
    #[grid_coords]
    grid_coords: GridCoords,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    pub collision_detection: CollisionDetection,
}

#[derive(Component, Default)]
pub struct Port {
    pub map: usize,
    pub location: Location,
}

impl Port {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        Self {
            map: *entity_instance
                .get_int_field("MapId")
                .expect("expected entity to have non-nullable MapID int field")
                as usize,
            location: Location(
                entity_instance
                    .get_string_field("Location")
                    .expect("expected entity to have non-nullable location String field")
                    .chars()
                    .next()
                    .expect("expected entity to have non-nullable non empty string field"),
            ),
        }
    }
}

#[derive(Component)]
pub struct Map;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct SpawnLocationBundle {
    #[with(name_from_location)]
    name: Name,
    #[with(Location::from_field)]
    location: Location,
    #[grid_coords]
    grid_coords: GridCoords,
}

fn name_from_location(_: &EntityInstance) -> Name {
    Name::new("Location")
}

#[derive(Default, Component, Clone, Debug)]
pub struct Location(char);

impl Location {
    pub fn new(location: char) -> Self {
        Self(location)
    }
}

impl Location {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        Self(
            entity_instance
                .get_string_field("Location")
                .expect("expected entity to have non-nullable location string field")
                .chars()
                .next()
                .unwrap(),
        )
    }

    pub fn equal(&self, location: &Location) -> bool {
        self.0 == location.0
    }
}

#[derive(Component, Clone, Reflect, Debug, PartialEq)]
pub enum Weather {
    Sunny,
    Rainy,
    Muddy,
    // pub timer: AnimationTimer,
    // pub anime: AnimationIndices,
}

impl Default for Weather {
    fn default() -> Self {
        Self::randomize()
    }
}

impl Weather {
    pub fn randomize() -> Self {
        let weather: Vec<Weather> = vec![Weather::Sunny, Weather::Rainy, Weather::Muddy]; // Define your enum variants here

        let mut rng = rand::thread_rng();
        let random_item: &Weather = weather.choose(&mut rng).unwrap();
        random_item.clone()
    }
}
