pub mod grid;

use crate::grid_plugin::grid::Grid;
use bevy::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let grid_side_length = 20;
    let x_space = 400.0;
    let grid = Grid::new(
        grid_side_length,
        grid_side_length,
        x_space,
        x_space,
        materials,
    );

    let batch = grid.matrix.map(|cell| {
        (
            DebugCube,
            Transform::from_xyz(cell.col, cell.row, 0.0),
            Mesh2d(meshes.add(Rectangle::new(10.0, 10.0))),
            MeshMaterial2d(cell.asset_id.clone()),
        )
    });
    commands.spawn_batch(batch);
    commands.insert_resource(grid);
}

#[derive(Component)]
pub struct DebugCube;

pub struct GridCell {
    temperature: f32,
    asset_id: Handle<ColorMaterial>,
    row: f32,
    col: f32,
}

pub trait ToKelvin {
    /// Convert from Celsius to Kelvin
    fn to_kelvin(celsius: f32) -> f32 {
        celsius + 273.15
    }
}
impl ToKelvin for f32 {}
