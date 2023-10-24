use std::ops::{Add, Div, Mul, Sub};

use crate::types::{Angle, Point2D, Point3D, UnitAngle};

impl Angle {
    /// Make a new angle in degrees
    pub fn from_degrees(size: f64) -> Self {
        Self {
            unit: UnitAngle::Degrees,
            value: size,
        }
    }
    /// Make a new angle in radians
    pub fn from_radians(size: f64) -> Self {
        Self {
            unit: UnitAngle::Radians,
            value: size,
        }
    }

    /// Get the size of the angle, in radians
    pub fn radians(&self) -> f64 {
        match self.unit {
            UnitAngle::Radians => self.value,
            UnitAngle::Degrees => self.value.to_radians(),
        }
    }

    /// Get the size of the angle, in degrees
    pub fn degrees(&self) -> f64 {
        match self.unit {
            UnitAngle::Degrees => self.value,
            UnitAngle::Radians => self.value.to_degrees(),
        }
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Point2D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div for Point2D {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Add for Point3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Point3D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div for Point3D {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Point3D {
    /// Add a Z component to a 2D point.
    pub fn with_z(Point2D { x, y }: Point2D, z: f64) -> Self {
        Self { x, y, z }
    }

    /// The origin
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

impl Point2D {
    /// The origin
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
}

impl From<Point3D> for Point2D {
    fn from(Point3D { x, y, .. }: Point3D) -> Self {
        Point2D { x, y }
    }
}
