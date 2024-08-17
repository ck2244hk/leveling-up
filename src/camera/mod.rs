use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    window::PrimaryWindow,
};

use crate::helper::{SmoothDamp, Velocity, Zooming};

pub mod game;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera);
    app.add_plugins(game::plugin);
}

/// Default render layers for pixel-perfect rendering.
// /// You can skip adding this component, as this is the default.
// const SHADOW_LAYERS: RenderLayers = RenderLayers::layer(0);

// /// Render layers for high-resolution rendering.
// const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

// const POSTPROCESS_LAYERS: RenderLayers = RenderLayers::layer(10);

/// Low-resolution texture that contains the pixel-perfect world.
/// Canvas itself is rendered to the high-resolution world.
// #[derive(Component)]
// struct Canvas;

// /// Camera that renders the pixel-perfect world to the [`Canvas`].
// #[derive(Component)]
// struct ShadowCamera;

/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
struct InGameCamera;

/// Camera that renders the [`Canvas`] (and other graphics on [`POSTPROCESS_LAYERS`]) to the screen.
// #[derive(Component)]
// struct PostCamera;

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(
    mut commands: Commands,
    // mut images: ResMut<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // meshes: ResMut<Assets<Mesh>>,
    // materials: ResMut<Assets<PostProcessingShader>>,
) {
    let window = window_query.single();

    let canvas_size = Extent3d {
        width: window.width().floor() as u32,
        height: window.height().floor() as u32,
        ..default()
    };

    let parent = commands
        .spawn((
            Name::new("Main Camera"),
            MainCamera,
            TransformBundle::default(),
            SmoothDamp::new(),
            Velocity(Vec2::ZERO),
        ))
        .id();

    // this Image serves as a canvas representing the low-resolution game screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    canvas.resize(canvas_size);

    // let image_handle = images.add(canvas);

    // this camera renders whatever is on `SHADOW_LAYERS` to the canvas
    // let shadow = commands
    //     .spawn((
    //         Name::new("Shadow Camera"),
    //         Camera2dBundle {
    //             camera: Camera {
    //                 // render before the "main pass" camera
    //                 order: 0,
    //                 target: RenderTarget::Image(image_handle.clone()),
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         ShadowCamera,
    //         SHADOW_LAYERS,
    //     ))
    //     .id();

    // the "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the a 2d Image.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    let in_game = commands
        .spawn((
            Name::new("Player Camera"),
            Zooming::default(),
            Camera2dBundle {
                camera: Camera {
                    // render before the "main pass" camera
                    // order: 1,
                    // target: RenderTarget::Image(image_handle.clone()),
                    ..default()
                },
                ..default()
            },
            InGameCamera,
        ))
        .id();

    // let mesh_handle = meshes.add(Rectangle::new(window.width(), window.height()));

    // let material_handle = materials.add(PostProcessingShader {
    //     center_pos: CenterPos { x: 0., y: 0. },
    //     img: image_handle,
    // });

    // spawn the canvas
    // let canvas = commands
    //     .spawn((
    //         MaterialMesh2dBundle {
    //             mesh: mesh_handle.into(),
    //             material: material_handle,
    //             ..default()
    //         },
    //         Canvas,
    //         POSTPROCESS_LAYERS,
    //     ))
    //     .id();

    // the "postprocess" camera renders whatever is on `Postprocessing` to the screen.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    // let post = commands
    //     .spawn((
    //         Name::new("Post Camera"),
    //         Camera2dBundle {
    //             camera: Camera {
    //                 order: 2,
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         PostCamera,
    //         POSTPROCESS_LAYERS,
    //     ))
    //     .id();

    commands.entity(parent).push_children(&[in_game]);
}
