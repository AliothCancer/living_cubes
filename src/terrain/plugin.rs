use bevy::prelude::*;

use super::Grid;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<WorldData>(WorldData {
            temperature: Grid::generate(100, 100, 20., 20.),
        })
        .add_systems(Startup, spawn_terrain);
    }
}

#[derive(Resource)]
struct WorldData {
    temperature: Grid,
}

fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world_data: Res<WorldData>,
) {
    let (min, max) = world_data.temperature.get_minmax();
    let origin = world_data.temperature.origin;
    let dx = world_data.temperature.dx;
    let dy = world_data.temperature.dy;

    world_data
        .temperature
        .values
        .rows()
        .into_iter()
        .enumerate()
        .for_each(|(y, row)| {
            let y = (y as f32) * dy;
            row.iter().enumerate().for_each(|(x, value)| {
                let x = (x as f32) * dx;

                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(10.0, 10.0))),
                    MeshMaterial2d(materials.add(compute_color((min, max), *value))),
                    Transform::from_xyz(origin.x + x, origin.y + y, 0.0),
                ));
            });
        });
}

fn compute_color(min_max: (f32, f32), value: f32) -> ColorMaterial {
    let (min, max) = min_max;
    let green = 0.0;
    let median = (max + min) / 2.;
    let red = ((value - median) / (max - median)).clamp(0.0, 1.0);
    let blue = ((median - value) / (median - min)).clamp(0.0, 1.0);
    ColorMaterial::from_color(Color::linear_rgba(red, green, blue, 1.))
}
