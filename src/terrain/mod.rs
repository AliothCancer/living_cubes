pub mod plugin;

use bevy::math::Vec2;
use ndarray::*;
use rand::rng;
use rand_distr::{Distribution, Uniform};

pub struct Grid {
    values: Array2<f32>,
    width: usize,
    height: usize,
    min: f32,
    max: f32,
    dx: f32,
    dy: f32,
    origin: Vec2,
}

pub trait ToKelvin {
    /// Convert from Celsius to Kelvin
    fn to_kelvin(celsius: f32) -> f32 {
        celsius + 273.15
    }
}
impl ToKelvin for f32 {}

impl Grid {
    pub fn generate(width: usize, height: usize, dx: f32, dy: f32) -> Grid {
        let mut rng = rng();
        let (min, max) = (f32::to_kelvin(15.0_f32), f32::to_kelvin(35.0_f32));
        let dist = Uniform::new(min, max).unwrap();
        let values = Array2::from_shape_simple_fn(Dim([width, height]), || dist.sample(&mut rng));
        // let (dx, dy) = (3.0, 3.0);

        let x = -(width as f32) / 2. * dx;
        let y = -(height as f32) / 2. * dy;
        Grid {
            values,
            width,
            height,
            min,
            max,
            dx,
            dy,
            origin: Vec2 { x, y },
        }
    }
    pub fn get_minmax(&self) -> (f32, f32) {
        (self.min, self.max)
    }
}
