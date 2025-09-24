use bevy::prelude::*;

use crate::cube_logic::{compute_color, grid::Grid};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        let dx = 400.0;
        let square_size = 20;
        app.insert_resource::<WorldData>(WorldData {
            temperature: Grid::generate(square_size, square_size, dx, dx),
        })
        .add_systems(Startup, spawn_terrain);
    }
}

#[derive(Resource)]
pub struct WorldData {
    pub temperature: Grid,
}
// #[derive(Component)]
// pub struct GridCube(pub AssetId<ColorMaterial>);

fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut world_data: ResMut<WorldData>,
) {
    let (min, max) = world_data.temperature.get_minmax();
    let _origin = world_data.temperature.origin;
    let dx = world_data.temperature.dx;
    let dy = world_data.temperature.dy;

    world_data
        .temperature
        .values
        .rows_mut()
        .into_iter()
        .enumerate()
        .for_each(|(x, mut row)| {
            let x = dx * ((x as f32) + 0.5);
            row.iter_mut()
                .enumerate()
                .for_each(|(y, (temperature, color_asset))| {
                    let y = dy * ((y as f32) + 0.5);
                    let color_handle = materials.add(compute_color((min, max), *temperature));
                    let asset_id = color_handle.id().to_owned();

                    // color asset should be None cuz Grid is created before Asset<ColorMaterial> is loaded,
                    // Currently the a new MeshMaterial2d is assigned to replace the color everytime coordinate
                    // match the one of gridCube
                    // here asset_id is available and we can assign it
                    if color_asset.is_none() {
                        *color_asset = Some(asset_id);
                    } else {
                        panic!("ColorAsset must be None")
                    }
                    commands.spawn((
                        // GridCube(asset_id),
                        Mesh2d(meshes.add(Rectangle::new(10.0, 10.0))),
                        MeshMaterial2d(color_handle),
                        Transform::from_xyz(x, y, 0.0),
                    ));
                });
        });
}
