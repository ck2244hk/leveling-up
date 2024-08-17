use bevy::{
    prelude::*,
    render::{
        camera,
        render_resource::{AsBindGroup, ShaderRef, ShaderType},
    },
    sprite::{Material2d, Material2dPlugin},
    window::PrimaryWindow,
};

use crate::game::{
    map::preload::Weather,
    player::{Player, PlayerEnv},
};

use crate::shader_util::component::TexHandleQueue;

// ************************************ //
//                2D                    //
// ************************************ //
/// The 2D shadertoy like shader
// #[uuid = "f528511f-dcf2-4b0b-9522-a9df3a1a795b"]
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct LensFlareShader {
    #[uniform(0)]
    pub(crate) center_pos: CenterPos,

    #[texture(1, dimension = "2d")]
    #[sampler(2)]
    pub img: Handle<Image>,
}

impl Material2d for LensFlareShader {
    fn fragment_shader() -> ShaderRef {
        "shaders/lens_flare.wgsl".into()
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct PostProcessingShader {
    #[uniform(0)]
    pub(crate) center_pos: CenterPos,

    #[texture(1, dimension = "2d")]
    #[sampler(2)]
    pub img: Handle<Image>,
}

impl Material2d for PostProcessingShader {
    fn fragment_shader() -> ShaderRef {
        "shaders/post_processing.wgsl".into()
    }
}

#[derive(ShaderType, Debug, Clone)]
pub struct CenterPos {
    pub x: f32,
    pub y: f32,
}

impl CenterPos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
pub struct WeatherTexture;

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TexHandleQueue>()
            .add_plugins(Material2dPlugin::<LensFlareShader>::default())
            .add_plugins(Material2dPlugin::<PostProcessingShader>::default())
            .add_systems(Startup, spawn_len_flare)
            .add_systems(
                Update,
                (
                    update_weather_texture_position,
                    update_len_flare_direction,
                    update_weather_w_player,
                ),
            );
    }
}

fn spawn_len_flare(mut commands: Commands) {
    commands.spawn((
        WeatherTexture,
        Name::new("Weather Texture"),
        VisibilityBundle::default(),
        TransformBundle::default(),
    ));
}

fn update_weather_w_player(
    player_query: Query<&PlayerEnv, (Changed<PlayerEnv>)>,
    mut commands: Commands,
    mut materials: ResMut<Assets<LensFlareShader>>,
    mut texture_query: Query<(Entity), With<WeatherTexture>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut user_textures: ResMut<TexHandleQueue>,
    window_query: Query<&Window, With<PrimaryWindow>>, // camera_query: Query<&Transform, With<Camera2d>>,
) {
    let Ok(player_env) = player_query.get_single() else {
        return;
    };

    let Ok(texture_entity) = texture_query.get_single() else {
        return;
    };

    let window = window_query.single();

    let texture: Handle<Image> = asset_server.load("textures/rgb_noise.png");
    user_textures.insert(0, texture.clone());

    match player_env.0 {
        Some(Weather::Sunny) => {
            let lens_flare = commands
                .spawn(bevy::sprite::MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Rectangle::new(window.width(), window.height()))
                        .into(),
                    material: materials.add(LensFlareShader {
                        img: texture,
                        center_pos: CenterPos {
                            x: 0.0f32,
                            y: 0.0f32,
                        },
                    }),

                    transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                    // .with_rotation(Quat::from_rotation_x(180.0)), //FIXME to avoid the rotate2D call in all shaders..
                    ..default()
                })
                .id();
            commands.entity(texture_entity).add_child(lens_flare);
        }
        _ => {
            commands.entity(texture_entity).despawn_descendants();
        }
    }
}

fn update_weather_texture_position(
    mut texture_query: Query<&mut Transform, With<WeatherTexture>>,
    camera_query: Query<&Transform, (With<Camera2d>, Without<WeatherTexture>)>,
) {
    let Ok(camera) = camera_query.get_single() else {
        return;
    };

    for mut texture in texture_query.iter_mut() {
        texture.translation = Vec3::new(camera.translation.x, camera.translation.y, 99.);
    }
}

/// Update mouse_pos for the 2D shader
pub fn update_len_flare_direction(
    shader_hndl: Query<&Handle<LensFlareShader>>,
    texture_query: Query<&Transform, With<WeatherTexture>>,
    mut shader_mat: ResMut<Assets<LensFlareShader>>,
    player_query: Query<&Transform, (With<Player>, Without<WeatherTexture>)>,
    // mon_spec: Res<MonitorsSpecs>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    let Ok(handle) = shader_hndl.get_single() else {
        return;
    };

    let Ok(texture) = texture_query.get_single() else {
        return;
    };

    // Is the mouse on our window?
    if let Some(shad_mat) = shader_mat.get_mut(handle) {
        shad_mat.center_pos = CenterPos::new(
            player.translation.x - texture.translation.x,
            player.translation.y - texture.translation.y,
        );
    }
}
