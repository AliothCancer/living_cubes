use bevy::prelude::*;
use ndarray::*;
use ndarray_linalg::Inverse;
use rand::rng;
use rand_distr::{Distribution, Uniform};

use crate::{
    cube::compute_color,
    grid_plugin::{
        GridPoint,
        coordinate::{
            GameCoor, GridCoor, GridStep, QualitativeRelPos, compute_distance,
            get_qualitative_position,
        },
    },
};

pub const COLS: usize = 20;
pub const ROWS: usize = COLS;
pub const X_SPACE: f32 = 500.0;
pub const Y_SPACE: f32 = X_SPACE;
pub const ORIGIN: Vec2 = Vec2 { x: 0.0, y: 0.0 };
pub const MIN_TEMP: f32 = 0.01;
pub const MAX_TEMP: f32 = 200.0;

#[derive(Resource)]
pub struct Grid {
    pub matrix: Array2<GridPoint>,
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct GridCell {
    pub bottom_right: Vec2,
    pub bottom_left: Vec2,
    pub top_left: Vec2,
    pub top_right: Vec2,
}
impl GridCell {
    pub fn points(&self) -> [Vec2; 4] {
        [
            self.bottom_left,
            self.top_left,
            self.top_right,
            self.bottom_right,
        ]
    }
}

const LEFT: GridStep = GridStep { x: -1, y: 0 };
const RIGHT: GridStep = GridStep { x: 1, y: 0 };
const TOP: GridStep = GridStep { x: 0, y: 1 };
const BOTTOM: GridStep = GridStep { x: 0, y: -1 };

fn bilinear_interpolation(bil_coeffs: Array1<f32>, x: f32, y: f32) -> f32 {
    bil_coeffs[0] + bil_coeffs[1] * x + bil_coeffs[2] * y + bil_coeffs[3] * x * y
}

impl Grid {
    pub fn interpolate_temperature(&self, cell: GridCell, coor: GameCoor) -> f32 {
        let points = cell.points();
        let temps = Array1::from_iter(
            points
                .into_iter()
                .map(|point| self.get_value(GridCoor::from(point)).temperature),
        );
        let coeffs = array![
            [
                1.0,
                cell.bottom_left.x,
                cell.bottom_left.y,
                cell.bottom_left.x * cell.bottom_left.y
            ],
            [
                1.0,
                cell.top_left.x,
                cell.top_left.y,
                cell.top_left.x * cell.top_left.y
            ],
            [
                1.0,
                cell.top_right.x,
                cell.top_right.y,
                cell.top_right.x * cell.top_right.y
            ],
            [
                1.0,
                cell.bottom_right.x,
                cell.bottom_right.y,
                cell.bottom_right.x * cell.bottom_right.y
            ],
        ];
        let bil_coeffs = dbg!(coeffs)
            .inv()
            .unwrap_or(Array::zeros((4, 4)))
            // .expect("Problem while inverting matrix")
            .dot(&temps);
        bilinear_interpolation(bil_coeffs, coor.x, coor.y)
    }
    pub fn get_cell_temp(&self, grid_cell: GridCell, coor: GameCoor) -> [(f32, f32); 4] {
        let temp_and_dist = |grid_cell_vertex: Vec2| {
            let grid_point = self.get_value(GridCoor::from(grid_cell.bottom_left));
            let temp = grid_point.temperature;
            let dist = compute_distance(coor, grid_cell_vertex);
            (temp, dist)
        };
        [
            dbg!(temp_and_dist(grid_cell.bottom_left)),
            dbg!(temp_and_dist(grid_cell.bottom_right)),
            dbg!(temp_and_dist(grid_cell.top_right)),
            dbg!(temp_and_dist(grid_cell.top_left)),
        ]
    }
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
