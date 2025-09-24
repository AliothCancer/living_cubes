use bevy::{asset::AssetId, log::info, math::Vec2, sprite::ColorMaterial};
use ndarray::*;
use rand::rng;
use rand_distr::{Distribution, Uniform};

use crate::cube_logic::{ColorWeights, NearCube, compute_color};

pub struct Grid {
    pub values: Array2<(f32, Option<AssetId<ColorMaterial>>)>,
    pub width: usize,
    pub height: usize,
    pub min: f32,
    pub max: f32,
    pub dx: f32,
    pub dy: f32,
    pub origin: Vec2,
}

pub trait ToKelvin {
    /// Convert from Celsius to Kelvin
    fn to_kelvin(celsius: f32) -> f32 {
        celsius + 273.15
    }
}
impl ToKelvin for f32 {}

impl Grid {
    pub fn get_value(&self, x: usize, y: usize) -> Option<&(f32, Option<AssetId<ColorMaterial>>)> {
        self.values.get((x, y))
    }
    pub fn get_dxdy(&self) -> (f32, f32) {
        (self.dx, self.dy)
    }
    pub fn generate(width: usize, height: usize, dx: f32, dy: f32) -> Grid {
        let mut rng = rng();
        let (min, max) = (f32::to_kelvin(15.0_f32), f32::to_kelvin(35.0_f32));
        let dist = Uniform::new(min, max).unwrap();
        let values =
            Array2::from_shape_simple_fn(Dim([width, height]), || (dist.sample(&mut rng), None));
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

    /// This func will return the adjacent elements of the given (x,y) element
    pub fn get_near_cubes(
        &self,
        x: usize,
        y: usize,
        quantity: AdjacentCubeQuantity,
        cube_coor: Vec2,
    ) -> Vec<NearCube> {
        let x_max = self.values.len_of(Axis(0)) as i32 - 1;
        let y_max = self.values.len_of(Axis(1)) as i32 - 1;

        match quantity {
            AdjacentCubeQuantity::ThreeByThree => {
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

                        let temperature = match self.get_value(x as usize, y as usize) {
                            Some(val) => val.0,
                            None => panic!("None on indexes {:?}", (x as usize, y as usize)),
                        };
                        let distance =
                            cube_coor.distance(Vec2::new(x as f32 * self.dx, y as f32 * self.dy));
                        let color = compute_color(self.get_minmax(), temperature)
                            .color
                            .to_srgba();
                        let red = color.red;
                        let blue = color.blue;
                        NearCube::new(ColorWeights { red, blue }, distance, temperature)
                    })
                    .collect::<Vec<NearCube>>()
            }
            AdjacentCubeQuantity::FourByFour => todo!(),
        }
    }
}

pub enum AdjacentCubeQuantity {
    ThreeByThree,
    FourByFour,
}
