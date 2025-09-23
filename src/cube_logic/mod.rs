use std::ops::{Add, Div};

use bevy::{color::Color, sprite::ColorMaterial};

pub mod grid;

/// This color is the one associated with one point of the grid, not the color of the cube
pub fn compute_color(min_max: (f32, f32), temperature: f32) -> ColorMaterial {
    let (min, max) = min_max;
    let green = 0.0;
    let median = (max + min) / 2.;
    let red = ((temperature - median) / (max - median)).clamp(0.0, 1.0);
    let blue = ((median - temperature) / (median - min)).clamp(0.0, 1.0);
    ColorMaterial::from_color(Color::linear_rgba(red, green, blue, 1.))
}

pub struct ColorWeights {
    red: f32,
    blue: f32,
}

impl ColorWeights {
    pub fn new(red: f32, blue: f32) -> Self {
        Self { red, blue }
    }
}
impl Add for ColorWeights {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let red = self.red + rhs.red;
        let blue = self.blue + rhs.blue;
        ColorWeights { red, blue }
    }
}

impl Div for ColorWeights {
    type Output = ColorWeights;

    fn div(self, rhs: Self) -> Self::Output {
        let red = self.red / rhs.red;
        let blue = self.blue / rhs.blue;
        ColorWeights { red, blue }
    }
}

/// Represents an arbitrary near (as distant as you want) temperature point of the grid
pub struct NearCube {
    color: ColorWeights,
    distance: f32,
}

/// TODO! Dovrei calcolare il colore solo sulla temperatura in modo simile a quello che ho fatto
/// con compute_colore
/// Compute the color of the cube based on the distance from near temperature points
/// as the geometric mean of the points
pub fn cube_temperature(near_cubes_color: Vec<NearCube>) -> ColorWeights {
    todo!();
    let mut total_distance = 0.0;
    near_cubes_color
        .iter()
        .map(|near_cube| {
            let red = near_cube.color.red;
            let blue = near_cube.color.blue;

            total_distance += near_cube.distance;

            ColorWeights::new(red * near_cube.distance, blue * near_cube.distance)
        })
        .fold(ColorWeights::new(0.0, 0.0), |acc, x| acc + x)
        .div(ColorWeights {
            red: total_distance,
            blue: total_distance,
        })
}
