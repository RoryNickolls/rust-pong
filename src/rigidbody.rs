use crate::transform::Transform;
use crate::vector::Vector;
use crate::bounds::Bounds;

pub struct Rigidbody {
    transform: Transform,
    velocity: Vector,
    bounds: Bounds,
}

impl Rigidbody {
    pub fn new(transform: Transform, velocity: Vector, bounds: Bounds) -> Rigidbody {
        Rigidbody { transform: transform, velocity: velocity, bounds: bounds }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn velocity(&self) -> Vector {
        self.velocity
    }

    pub fn velocity_mut(&mut self) -> &mut Vector {
        &mut self.velocity
    }
}