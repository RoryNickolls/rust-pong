use crate::transform::Transform;
use crate::vector::Vector;
use crate::update::Update;
use crate::render::Vertex;

pub trait Collide {
    fn on_collide(&mut self, mut other: Collide);
}

pub struct Rigidbody {
    pub transform: Transform,
    pub velocity: Vector,
}

impl Rigidbody {
    pub fn new() -> Rigidbody {
        Rigidbody { transform: Transform::new(), velocity: Vector::zero() }
    }
}

impl Update for Rigidbody {
    fn update(&mut self, delta_time: f32) {
        self.transform.position.x += self.velocity.x * delta_time;
        self.transform.position.y += self.velocity.y * delta_time;
    }
}