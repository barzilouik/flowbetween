use curves::*;

use std::f32;
use std::ops::{Mul, Add, Sub};

///
/// A point in a path
/// 
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PathPoint {
    /// X, Y coordinates of this point
    pub position: (f32, f32)
}

impl PathPoint {
    ///
    /// Creates a new path point
    /// 
    pub fn new(x: f32, y: f32) -> PathPoint {
        PathPoint {
            position: (x, y)
        }
    }

    pub fn x(&self) -> f32 {
        self.position.0
    }

    pub fn y(&self) -> f32 {
        self.position.1
    }
}

impl Add<PathPoint> for PathPoint {
    type Output=PathPoint;

    #[inline]
    fn add(self, rhs: PathPoint) -> PathPoint {
        PathPoint {
            position: (self.position.0 + rhs.position.0, self.position.1 + rhs.position.1)
        }
    }
}

impl Sub<PathPoint> for PathPoint {
    type Output=PathPoint;

    #[inline]
    fn sub(self, rhs: PathPoint) -> PathPoint {
        PathPoint {
            position: (self.position.0 - rhs.position.0, self.position.1 - rhs.position.1)
        }
    }
}

impl Mul<f64> for PathPoint {
    type Output=PathPoint;

    #[inline]
    fn mul(self, rhs: f64) -> PathPoint {
        let rhs = rhs as f32;

        PathPoint {
            position: (self.position.0 * rhs, self.position.1 * rhs)
        }
    }
}

impl Coordinate for PathPoint {
    ///
    /// Creates a new coordinate from the specified set of components
    /// 
    #[inline]
    fn from_components(components: &[f64]) -> Self {
        PathPoint {
            position: (components[0] as f32, components[1] as f32)
        }
    }

    ///
    /// Returns the origin coordinate
    /// 
    #[inline]
    fn origin() -> Self {
        PathPoint {
            position: (0.0,0.0)
        }
    }

    ///
    /// The number of components in this coordinate
    /// 
    fn len() -> usize {
        2
    }

    ///
    /// Retrieves the component at the specified index
    /// 
    #[inline]
    fn get(&self, index: usize) -> f64 {
        match index {
            0 => self.position.0 as f64,
            1 => self.position.1 as f64,

            _ => 0.0
        }
    }

    ///
    /// Returns a point made up of the biggest components of the two points
    /// 
    #[inline]
    fn from_biggest_components(p1: Self, p2: Self) -> Self {
        PathPoint {
            position: (
                f32::max(p1.position.0, p2.position.0),
                f32::max(p1.position.1, p2.position.1)
            )
        }
    }

    ///
    /// Returns a point made up of the smallest components of the two points
    ///
    #[inline]
    fn from_smallest_components(p1: Self, p2: Self) -> Self {
        PathPoint {
            position: (
                f32::min(p1.position.0, p2.position.0),
                f32::min(p1.position.1, p2.position.1)
            )
        }
    }
}

impl Coordinate2D for PathPoint {
    #[inline]
    fn x(&self) -> f64 { self.position.0 as f64 }

    #[inline]
    fn y(&self) -> f64 { self.position.1 as f64 }
}