use std::iter::Zip;

use rand_distr::num_traits::Inv;

use crate::grid_plugin::grid::GridCoor;

pub trait WeightedTemperature {
    fn weighted_temperature(self) -> f32;
}

impl<Temperatures, Distances> WeightedTemperature for Zip<Temperatures, Distances>
where
    Temperatures: Iterator<Item = f32> + ExactSizeIterator,
    Distances: Iterator<Item = f32> + ExactSizeIterator,
{
    /// The len should be like 16 which are the points of the grid
    /// the len depends on how many points is decided to compute the
    /// temperature with
    fn weighted_temperature(self) -> f32 {
        (self.len() as f32).inv() * self.map(|(temp, dist)| temp * dist).sum::<f32>()
    }
}
