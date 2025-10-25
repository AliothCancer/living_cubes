pub mod coordinate;
pub mod grid;
pub mod temperature;

use crate::grid_plugin::{
    coordinate::{GameCoor, GridCoor},
    grid::Grid,
};
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
    let grid = Grid::new(materials);

    let batch = grid.matrix.map(|cell| {
        (
            DebugCube,
            Transform::from_xyz(cell.game_coor.x, cell.game_coor.y, 0.0),
            Mesh2d(meshes.add(Rectangle::new(10.0, 10.0))),
            MeshMaterial2d(cell.asset_id.clone()),
        )
    });
    commands.spawn_batch(batch);
    commands.insert_resource(grid);
}

#[derive(Component)]
pub struct DebugCube;

pub struct GridPoint {
    pub temperature: f32,
    pub asset_id: Handle<ColorMaterial>,
    pub game_coor: GameCoor,
    pub grid_coor: GridCoor,
}

pub trait ToKelvin {
    /// Convert from Celsius to Kelvin
    fn to_kelvin(celsius: f32) -> f32 {
        celsius + 273.15
    }
}
impl ToKelvin for f32 {}
