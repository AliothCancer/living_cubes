use super::ToKelvin;
use bevy::prelude::*;
use ndarray::*;
use rand::rng;
use rand_distr::{Distribution, Uniform};

use crate::{
    cube::{ColorWeights, NearCube, compute_color},
    grid_plugin::{
        GridPoint,
        coordinate::{GameCoor, GridCoor, GridStep, QualitativeRelPos, get_qualitative_position},
    },
};

pub const COLS: usize = 20;
pub const ROWS: usize = COLS;
pub const X_SPACE: f32 = 100.0;
pub const Y_SPACE: f32 = X_SPACE;
pub const ORIGIN: Vec2 = Vec2 { x: 0.0, y: 0.0 };
pub const MIN_TEMP: f32 = 15.0;
pub const MAX_TEMP: f32 = 35.0;

#[derive(Resource)]
pub struct Grid {
    pub matrix: Array2<GridPoint>,
}

#[derive(Debug, PartialEq, Default)]
pub struct GridCell {
    pub bottom_right: Vec2,
    pub bottom_left: Vec2,
    pub top_left: Vec2,
    pub top_right: Vec2,
}
const LEFT: GridStep = GridStep { x: -1, y: 0 };
const RIGHT: GridStep = GridStep { x: 1, y: 0 };
const TOP: GridStep = GridStep { x: 0, y: 1 };
const BOTTOM: GridStep = GridStep { x: 0, y: -1 };

impl Grid {
    /// A cell is formed of 4 points, each of which is adjacent and forms a square
    pub fn find_nearest_cell(&self, cube_coor: Vec3) -> GridCell {
        let cube_coor_game = GameCoor::from(cube_coor);
        let cube_coor_grid = GridCoor::from(cube_coor_game);
        let rel_pos = get_qualitative_position(cube_coor_game, cube_coor_grid);

        match rel_pos {
            QualitativeRelPos::TopLeft => GridCell {
                top_left: (cube_coor_grid + TOP + LEFT).into(),
                top_right: (cube_coor_grid + TOP).into(),
                bottom_left: (cube_coor_grid + LEFT).into(),
                bottom_right: (cube_coor_grid).into(),
            },
            QualitativeRelPos::TopRight => GridCell {
                top_right: (cube_coor_grid + TOP + RIGHT).into(),
                top_left: (cube_coor_grid + TOP).into(),
                bottom_right: (cube_coor_grid + RIGHT).into(),
                bottom_left: (cube_coor_grid).into(),
            },
            QualitativeRelPos::BottomLeft => GridCell {
                bottom_left: (cube_coor_grid + BOTTOM + LEFT).into(),
                top_left: (cube_coor_grid + LEFT).into(),
                bottom_right: (cube_coor_grid + BOTTOM).into(),
                top_right: (cube_coor_grid).into(),
            },
            QualitativeRelPos::BottomRight => GridCell {
                bottom_right: (cube_coor_grid + BOTTOM + RIGHT).into(),
                bottom_left: (cube_coor_grid + BOTTOM).into(),
                top_right: (cube_coor_grid + RIGHT).into(),
                top_left: (cube_coor_grid).into(),
            },
        }
    }
    pub fn get_value(&self, coor: GridCoor) -> &GridPoint {
        self.matrix.get((coor.x, coor.y)).unwrap()
    }

    pub fn new(mut materials: ResMut<Assets<ColorMaterial>>) -> Grid {
        let matrix = {
            let mut rng = rng();
            let distr = Uniform::new(MIN_TEMP, MAX_TEMP).unwrap();
            Array2::from_shape_fn(Dim([COLS, ROWS]), |(row, col)| {
                let temperature = distr.sample(&mut rng);
                let color = compute_color(temperature);
                let asset_id = materials.add(ColorMaterial::from_color(color));
                let y = row as f32 * X_SPACE;
                let x = col as f32 * Y_SPACE;
                GridPoint {
                    temperature,
                    asset_id,
                    grid_coor: GridCoor { y: row, x: col },
                    game_coor: GameCoor { x, y },
                }
            })
        };
        Grid { matrix }
    }
}
