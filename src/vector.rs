use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Neg;
use std::ops::Mul;

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    pub fn zero() -> Vector {
        Vector { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn one() ->  Vector {
        Vector { x: 1.0, y: 1.0, z: 1.0 }
    }

    pub fn up() -> Vector {
        Vector { x: 0.0, y: 1.0, z: 0.0 }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector {

    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vector {

    fn sub_assign(&mut self, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, other: f32) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}