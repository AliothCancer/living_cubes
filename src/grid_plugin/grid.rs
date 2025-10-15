use std::iter::FilterMap;

use super::ToKelvin;
use bevy::prelude::*;
use ndarray::*;
use rand::rng;
use rand_distr::{Distribution, Uniform};

use crate::{
    cube::{ColorWeights, NearCube, compute_color},
    grid_plugin::GridCell,
};

#[derive(Resource)]
pub struct Grid {
    pub matrix: Array2<GridCell>,
    /// number of columns
    pub cols: usize,
    /// number of rows
    pub rows: usize,
    pub min: f32,
    pub max: f32,
    pub dx: f32,
    pub dy: f32,
    pub origin: Vec2,
}

impl Grid {
    pub fn get_value(&self, x: usize, y: usize) -> Option<&GridCell> {
        self.matrix.get((x, y))
    }

    pub fn new(
        cols: usize,
        rows: usize,
        dx: f32,
        dy: f32,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> Grid {
        let (min, max) = (f32::to_kelvin(15.0_f32), f32::to_kelvin(35.0_f32));
        let matrix = {
            let mut rng = rng();
            let distr = Uniform::new(min, max).unwrap();
            Array2::from_shape_fn(Dim([cols, rows]), |(row, col)| {
                let temperature = distr.sample(&mut rng);
                let color = compute_color((min, max), temperature);
                let asset_id = materials.add(ColorMaterial::from_color(color));
                let y = row as f32 * dy;
                let x = col as f32 * dx;
                GridCell {
                    temperature,
                    asset_id,
                    row,
                    col,
                    x,
                    y,
                }
            })
        };

        let origin = Vec2 {
            x: -(cols as f32) / 2. * dx,
            y: -(rows as f32) / 2. * dy,
        };
        Grid {
            matrix,
            cols,
            rows,
            min,
            max,
            dx,
            dy,
            origin,
        }
    }

    pub fn get_minmax(&self) -> (f32, f32) {
        (self.min, self.max)
    }
    /// This func will return the adjacent elements of the given (x,y) element
    pub fn get_near_cubes<'a>(
        &'a self,
        x: usize,
        y: usize,
        quantity: AdjacentCubeQuantity,
        cube_coor: Vec2,
    ) -> FilterMap<
        std::array::IntoIter<[isize; 2], 8>,
        impl FnMut([isize; 2]) -> Option<&'a GridCell>,
    > {
        ADJACENT_POSITIONS.into_iter().filter_map(move |rel| {
            let row = x
                .checked_add_signed(rel[0])
                .expect(" Col got overflow/underflow");
            let col = y
                .checked_add_signed(rel[1])
                .expect("Col got overflow/underflow");
            self.matrix.get((row, col))
        })

        // let x_max = self.cols as i64 - 1;
        // let y_max = self.rows as i64 - 1;
        // match quantity {
        //     AdjacentCubeQuantity::ThreeByThree => {
        //         three_by_three((x, y), (x_max, y_max), self, cube_coor)
        //     }
        //     AdjacentCubeQuantity::FourByFour => todo!(),
        // }
    }
}
const ADJACENT_POSITIONS: [[isize; 2]; 8] = [
    // positives
    [0, 1],
    [1, 0],
    [1, 1],
    // negatives
    [0, -1],
    [-1, 0],
    [-1, -1],
    // discordant
    [1, -1],
    [-1, 1],
];
fn three_by_three(
    (x, y): (usize, usize),
    (x_max, y_max): (i32, i32),
    grid_ref: &Grid,
    cube_coor: Vec2,
) -> Vec<NearCube> {
    let relative_indexes =
        (-2..=2).flat_map(|x_rel: i32| (-2..=2).map(move |y_rel: i32| (x_rel, y_rel)));
    relative_indexes
        .map(|(x_rel, y_rel)| {
            let x = match x as i32 + x_rel {
                ..=0 => 0,
                val if val > x_max => x_max,
                val => val,
            };
            let y = match y as i32 + y_rel {
                ..=0 => 0,
                val if val > y_max => y_max,
                val => val,
            };

            let temperature = match grid_ref.get_value(x as usize, y as usize) {
                Some(cell) => cell.temperature,
                None => panic!("None on indexes {:?}", (x as usize, y as usize)),
            };
            let distance =
                cube_coor.distance(Vec2::new(x as f32 * grid_ref.dx, y as f32 * grid_ref.dy));
            let mut red = 0.;
            let mut blue = 0.;
            if let Color::Srgba(color) = compute_color(grid_ref.get_minmax(), temperature) {
                red = color.red;
                blue = color.blue;
            }
            NearCube::new(ColorWeights { red, blue }, distance, temperature)
        })
        .collect::<Vec<NearCube>>()
}

pub enum AdjacentCubeQuantity {
    ThreeByThree,
    FourByFour,
}
