mod cube_logic;
mod grid_plugin;

use crate::{
    cube_logic::{compute_color, cube_color},
    grid_plugin::{
        GridPlugin,
        grid::{AdjacentCubeQuantity, Grid},
    },
};
use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    input::mouse::AccumulatedMouseScroll,
    prelude::*,
};

fn main() {
    App::new()
        //.insert_resource(ClearColor(Color::WHITE))
        .add_plugins((DefaultPlugins, GridPlugin))
        .add_systems(Startup, (setup_camera, spawn_cube))
        .add_systems(Update, update_camera)
        .add_systems(Update, (move_cube, update_color_cube))
        .run();
}

#[derive(Component)]
struct Cube {
    color_id: Handle<ColorMaterial>,
}
fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color_id = materials.add(ColorMaterial::from_color(Color::srgba(1.0, 1.0, 1.0, 1.0)));
    let size = 30.0;
    commands.spawn((
        Cube {
            color_id: color_id.clone(),
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh2d(meshes.add(Rectangle::new(size, size))),
        MeshMaterial2d(color_id),
    ));
}

fn move_cube(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cube_query: Query<&mut Transform, With<Cube>>,
) {
    let mut cube_transform = cube_query.single_mut().unwrap();
    // update cube color
    // 1. Use cube coordinate to find the point in the grid
    // move cube
    let speed = 5.0;
    if keyboard_input.pressed(KeyCode::KeyW) {
        cube_transform.translation.y += speed;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        cube_transform.translation.y -= speed;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        cube_transform.translation.x -= speed;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        cube_transform.translation.x += speed;
    }
}

fn update_color_cube(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cube_query: Query<(&mut Cube, &mut Transform)>,
    grid: Res<Grid>,
) {
    let (cube, transform) = cube_query.single_mut().unwrap();
    // update cube color
    // 1. Use cube coordinate to find the point in the grid
    let cube_coor = transform.translation;

    // posizione del valore di temperatura nell'array della grid
    let col = (cube_coor.x / grid.dx) as usize;
    let row = (cube_coor.y / grid.dy) as usize;
    // info!("{:?}", (x_index, y_index));
    // 2. Use the temperature of the point to colorize the cube

    // Update the color cube's Material mutating
    // it from Material Resources accessing
    // using the `Handle` save in GridCell

    if let Some(mat) = materials.get_mut(&cube.color_id) {
        mat.color = grid.compute_color();
    }
}

fn setup_camera(mut commands: Commands) {
    let mut persp = OrthographicProjection::default_2d();
    persp.scale = 0.1;
    commands.spawn((
        Projection::from(persp),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Camera2d,
        Camera {
            hdr: true, // 1. HDR is required for bloom
            ..default()
        },
        Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
        Bloom::default(),           // 3. Enable bloom for the camera
    ));
}

fn update_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    tracking_obj: Query<&Transform, (With<Cube>, Without<Camera2d>)>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    // time: Res<Time>,
) {
    let mut cam_transform = camera_query.single_mut().unwrap();
    cam_transform.translation = tracking_obj.single().unwrap().translation;
    let speed = 5.0;
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        cam_transform.translation.y += speed;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        cam_transform.translation.y -= speed;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        cam_transform.translation.x -= speed;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        cam_transform.translation.x += speed;
    }

    // CAMERA ZOOM SCROLL
    let vel = 0.1;
    let scroll_unit = mouse_scroll.delta.y;
    if scroll_unit.abs() > 0.0 {
        let delta = scroll_unit * vel;
        let (x, y, z) = (
            cam_transform.scale.x,
            cam_transform.scale.y,
            cam_transform.scale.z,
        );
        cam_transform.scale = cam_transform.scale.lerp(
            Vec3 {
                x: (x - delta).clamp(0.1, 30.0),
                y: (y - delta).clamp(0.1, 30.0),
                z,
            },
            0.9,
        );
    }

    //println!("x:{}, y{}", cam_transform.scale.x, cam_transform.scale.y)
}
