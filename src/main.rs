mod terrain;

use bevy::{core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}, input::mouse::AccumulatedMouseScroll, prelude::*};
use crate::terrain::plugin::TerrainPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, TerrainPlugin))
    .add_systems(Startup, setup_camera)
    .add_systems(Update, update_camera)
    .run();
}


fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 1.0),
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
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
) {
    let mut cam_transform = camera_query.single_mut().unwrap();

    // CAMERA ZOOM SCROLL
    let vel = 5.8;
    let scroll_unit = mouse_scroll.delta.y;
    if scroll_unit.abs() > 0.0 {
        let delta = scroll_unit * time.delta_secs() * vel;
        let (x, y, z) = (
            cam_transform.scale.x,
            cam_transform.scale.y,
            cam_transform.scale.z,
        );
        cam_transform.scale = cam_transform.scale.lerp(
            Vec3 {
                x: x - delta,
                y: y - delta,
                z,
            },
            0.9,
        );
    }

    //println!("x:{}, y{}", cam_transform.scale.x, cam_transform.scale.y)
}