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
