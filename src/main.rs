mod cube;
mod grid_plugin;

use crate::{
    cube::compute_color,
    grid_plugin::{
        GridPlugin,
        coordinate::GameCoor,
        grid::{Grid, GridCell, X_SPACE, Y_SPACE},
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
        .add_systems(Update, update_cube)
        .run();
}

#[derive(Component)]
struct Cube {
    color_id: Handle<ColorMaterial>,
    nearest_cell: GridCell,
}
fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color_id = materials.add(ColorMaterial::from_color(Color::srgba(1.0, 1.0, 1.0, 1.0)));
    let size = 50.0;
    commands.spawn((
        Cube {
            color_id: color_id.clone(),
            nearest_cell: GridCell::default(),
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh2d(meshes.add(Rectangle::new(size, size))),
        MeshMaterial2d(color_id),
    ));
}

fn move_cube(keyboard_input: Res<ButtonInput<KeyCode>>, translation: &mut Vec3) {
    let speed = 5.0;
    if keyboard_input.pressed(KeyCode::KeyW) {
        translation.y += speed;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        translation.y -= speed;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        translation.x -= speed;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        translation.x += speed;
    }
}

fn update_cube(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cube_query: Query<(&mut Cube, &mut Transform)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    grid: Res<Grid>,
) {
    let (mut cube, mut transform) = cube_query.single_mut().unwrap();

    move_cube(keyboard_input, &mut transform.translation);
    // update cube color
    // 1. Use cube coordinate to find the point in the grid
    let cube_coor = transform.translation;
    let current_cell = grid.find_nearest_cell(cube_coor);
    let prev_cell = &cube.nearest_cell;
    if prev_cell != &current_cell {
        // dbg!(cube_coor);
        // info!("spawned new debug black cube");
        // commands.spawn((
        //     Transform::from_xyz(
        //         current_cell.bottom_left.x + X_SPACE / 2.,
        //         current_cell.bottom_left.y + Y_SPACE / 2.,
        //         -1.0,
        //     ),
        //     Mesh2d(meshes.add(Rectangle::new(X_SPACE, Y_SPACE))),
        //     MeshMaterial2d(materials.add(Color::BLACK)),
        // ));
        cube.nearest_cell = current_cell;
    }
    let temp = grid.interpolate_temperature(current_cell, GameCoor::from(cube_coor));
    dbg!(temp);
    update_cube_color(materials, &cube.color_id, compute_color(temp));
    // TODO!
    // - Get the sub-grid The GridCells to compute the distace weighted temperature
    // - Compute the color
    // - Update the color of the cube
}

/// The piece of code which actually update the color of the cube
fn update_cube_color(
    mut materials: ResMut<Assets<ColorMaterial>>,
    cube_color_id: &Handle<ColorMaterial>,
    new_color: Color,
) {
    if let Some(mat) = materials.get_mut(cube_color_id) {
        mat.color = new_color;
    }
}

fn setup_camera(mut commands: Commands) {
    let mut persp = OrthographicProjection::default_2d();
    persp.scale = 2.1;
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
