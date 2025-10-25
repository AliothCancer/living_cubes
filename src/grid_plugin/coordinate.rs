use std::ops::Add;

use bevy::math::{Vec2, Vec3};

use super::grid::{X_SPACE, Y_SPACE};

pub struct GridStep {
    pub x: isize,
    pub y: isize,
}
impl Add<GridStep> for GridCoor {
    type Output = GridCoor;

    fn add(self, rhs: GridStep) -> Self::Output {
        GridCoor {
            x: self.x.checked_add_signed(rhs.x).unwrap_or(0),
            y: self.y.checked_add_signed(rhs.y).unwrap_or(0),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct GridCoor {
    pub x: usize,
    pub y: usize,
}
#[derive(Debug, Clone, Copy)]
pub struct GameCoor {
    pub x: f32,
    pub y: f32,
}

impl From<Vec3> for GameCoor {
    fn from(value: Vec3) -> Self {
        GameCoor {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<Vec2> for GameCoor {
    fn from(value: Vec2) -> Self {
        GameCoor {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Vec2> for GridCoor {
    fn from(value: Vec2) -> Self {
        GridCoor::from(GameCoor::from(value))
    }
}
impl From<Vec3> for GridCoor {
    fn from(value: Vec3) -> Self {
        GridCoor::from(GameCoor::from(value))
    }
}

impl From<GameCoor> for GridCoor {
    fn from(value: GameCoor) -> Self {
        GridCoor {
            x: (value.x / X_SPACE) as usize,
            y: (value.y / Y_SPACE) as usize,
        }
    }
}

impl From<GridCoor> for Vec2 {
    fn from(value: GridCoor) -> Self {
        Vec2 {
            x: value.x as f32 * X_SPACE,
            y: value.y as f32 * Y_SPACE,
        }
    }
}

pub enum QualitativeRelPos {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
/// Will return the position of the entity(i.e. cube)
/// relative to the grid point, it is usefull to understand
/// in which gridCell the cube is in
pub fn get_qualitative_position(coor_game: GameCoor, coor_grid: GridCoor) -> QualitativeRelPos {
    let grid_coor_x = coor_grid.x as f32;
    let grid_coor_y = coor_grid.y as f32;

    let is_left = coor_game.x <= grid_coor_x;
    let is_right = !is_left;
    let is_bottom = coor_game.y <= grid_coor_y;
    let is_top = !is_bottom;

    if is_bottom && is_left {
        QualitativeRelPos::BottomLeft
    } else if is_bottom && is_right {
        QualitativeRelPos::BottomRight
    } else if is_top && is_left {
        QualitativeRelPos::TopLeft
    } else if is_top && is_right {
        QualitativeRelPos::TopRight
    } else {
        unreachable!()
    }
}

pub fn compute_distance(a: GameCoor, b: Vec2) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}
