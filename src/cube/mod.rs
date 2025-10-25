use std::ops::{Add, Div};

use bevy::color::Color;

use crate::grid_plugin::grid::{MAX_TEMP, MIN_TEMP};

pub fn compute_color(temperature: f32) -> Color {
    let green = 0.0;
    let median = (MAX_TEMP + MIN_TEMP) / 2.0;
    let red = ((temperature - median) / (MAX_TEMP - median)).clamp(0.0, 3.0);
    let blue = ((median - temperature) / (median - MIN_TEMP)).clamp(0.0, 3.0);

    // Intensità basata su quanto ci si allontana dalla mediana
    let distance_from_median = (temperature - median).abs() / ((MAX_TEMP - MIN_TEMP) / 2.0);
    let bloom_multiplier = 1.0 + distance_from_median * 3.0; // Da 1.0 a 4.0

    Color::srgba(red * bloom_multiplier, green, blue * bloom_multiplier, 0.7)
}

pub struct ColorWeights {
    pub red: f32,
    pub blue: f32,
}

impl ColorWeights {
    pub fn _new(red: f32, blue: f32) -> Self {
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
    temperature: f32,
}

impl NearCube {
    pub fn new(color: ColorWeights, distance: f32, temperature: f32) -> Self {
        Self {
            color,
            distance,
            temperature,
        }
    }
}

/// Compute the color of the cube based on the distance from near temperature points
/// as the geometric mean of the points
pub fn cube_color(min_max: (f32, f32), near_cubes: Vec<NearCube>) -> Color {
    let mut total_temp_dist_ratios = 0.0;
    let mut total_inverse_distance = 0.0;

    for cube in near_cubes.iter() {
        if cube.distance < 0.1 {
            return Color::srgba(cube.color.red, 0.0, cube.color.blue, 0.9);
        }

        total_temp_dist_ratios += cube.temperature / cube.distance;
        total_inverse_distance += 1.0 / cube.distance;
    }

    let mean_temperature = total_temp_dist_ratios / total_inverse_distance;
    compute_color(mean_temperature)
}

/* old algorithm
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
        });

*/
