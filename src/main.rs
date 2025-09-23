mod cube_logic;
mod terrain;

use crate::terrain::plugin::{TerrainPlugin, WorldData};
use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    input::mouse::AccumulatedMouseScroll,
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerrainPlugin))
        .add_systems(Startup, (setup_camera, spawn_cube))
        .add_systems(Update, (move_cube, update_camera))
        .run();
}

#[derive(Component)]
struct Cube;
fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Cube,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh2d(meshes.add(Rectangle::new(30.0, 30.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(LinearRgba::new(
            1.0, 1.0, 1.0, 1.0,
        )))),
    ));
}

fn move_cube(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    materials: Res<Assets<ColorMaterial>>,
    mut cube_query: Query<(&mut MeshMaterial2d<ColorMaterial>, &mut Transform, Entity), With<Cube>>,
    world_data: Res<WorldData>,
) {
    let (mut color_mat, mut cube_transform, cube_entity) = cube_query.single_mut().unwrap();

    // update cube color
    // 1. Use cube coordinate to find the point in the grid
    let cube_coor = cube_transform.translation;
    let (dx, dy) = world_data.temperature.get_dxdy();

    // posizione del valore di temperatura nell'array della grid
    let point_x = cube_coor.x / dx;
    let point_y = cube_coor.y / dy;

    // 2. Use the temperature of the point to colorize the cube
    if let Some(local_temperature) = world_data.temperature.get_value(point_x, point_y) {
        let id = local_temperature.1.unwrap();
        commands
            .entity(cube_entity)
            .insert(MeshMaterial2d(Handle::Weak(id)));
    }

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

fn setup_camera(mut commands: Commands) {
    commands.spawn((
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
    mouse_scroll: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
) {
    let mut cam_transform = camera_query.single_mut().unwrap();

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
