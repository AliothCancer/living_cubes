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
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let grid_side_length = 20;
    let x_space = 400.0;
    let grid = Grid::new(
        grid_side_length,
        grid_side_length,
        x_space,
        x_space,
        meshes,
        materials,
    );
    commands.insert_resource(grid);

    grid.matrix.map(|cell|{
        (DebugCube, Transform)
    });
}

#[derive(Component)]
pub DebugCube;

pub struct GridCell {
    temperature: f32,
    asset_id: Handle<ColorMaterial>,
}

pub trait ToKelvin {
    /// Convert from Celsius to Kelvin
    fn to_kelvin(celsius: f32) -> f32 {
        celsius + 273.15
    }
}
impl ToKelvin for f32 {}
