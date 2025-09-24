mod cube_logic;
mod terrain;

use crate::{
    cube_logic::cube_color,
    terrain::plugin::{TerrainPlugin, WorldData},
};
use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    input::mouse::AccumulatedMouseScroll,
    prelude::*,
};

fn main() {
    App::new()
        //.insert_resource(ClearColor(Color::WHITE))
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
    let size = 300.0;
    commands.spawn((
        Cube,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh2d(meshes.add(Rectangle::new(size, size))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgba(
            1.0, 1.0, 1.0, 1.0,
        )))),
    ));
}

fn move_cube(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cube_query: Query<(&mut MeshMaterial2d<ColorMaterial>, &mut Transform, Entity), With<Cube>>,
    world_data: Res<WorldData>,
) {
    let (_color_mat, mut cube_transform, cube_entity) = cube_query.single_mut().unwrap();

    // update cube color
    // 1. Use cube coordinate to find the point in the grid
    let cube_coor = cube_transform.translation;
    let (dx, dy) = world_data.temperature.get_dxdy();

    // posizione del valore di temperatura nell'array della grid
    let x_index = (cube_coor.x / dx) as usize;
    let y_index = (cube_coor.y / dy) as usize;
    // info!("{:?}", (x_index, y_index));
    // 2. Use the temperature of the point to colorize the cube
    let near_cubes = world_data.temperature.get_near_cubes(
        x_index,
        y_index,
        cube_logic::grid::AdjacentCubeQuantity::ThreeByThree,
        Vec2::new(cube_coor.x, cube_coor.y),
    );
    let color = cube_color(world_data.temperature.get_minmax(), near_cubes);
    if let Some(_local_temperature) = world_data.temperature.get_value(x_index, y_index) {
        // let id = local_temperature.1.unwrap();

        commands
            .entity(cube_entity)
            .insert(MeshMaterial2d(materials.add(color)));
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
    tracking_obj: Query<&Transform, (With<Cube>, Without<Camera2d>)>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    // time: Res<Time>,
) {
    let mut cam_transform = camera_query.single_mut().unwrap();
    cam_transform.translation = tracking_obj.single().unwrap().translation.clone();
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
