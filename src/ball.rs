use crate::rigidbody::Rigidbody;
use crate::render::{Render, Vertex};
use crate::update::Update;
use crate::transform::Transform;
use crate::vector::Vector;
use crate::bounds::Bounds;

pub struct Ball {
    pub transform: Transform,
    pub velocity: Vector,
    pub in_play: bool,
}

impl Ball {
    pub fn new() -> Ball {
        Ball { transform: Transform::new(), velocity: Vector::zero(), in_play: true, }
    }
}

impl Rigidbody for Ball {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn velocity(&self) -> &Vector {
        &self.velocity
    }

    fn bounds(&self) -> Bounds {
        Bounds::new(self.transform.position, Vector::zero())
    }

    fn on_collide<R: Rigidbody>(&mut self, other: Box<R>) {

    }
}

impl Update for Ball {
    fn update(&mut self, delta_time: f32) {
        if self.transform.position.y <= -1.0 {
            self.transform.position.y = 1.0;
        }
    }
}

impl Render for Ball {
    fn model_matrix(&self) -> [[f32; 4]; 4] {
        self.transform.transform_matrix()
    }

    fn vertices() -> Vec<Vertex> {
        vec!
        [
            Vertex { position: [ -0.2,  0.2, 0.0 ] },
            Vertex { position: [  0.2,  0.2, 0.0 ] },
            Vertex { position: [ -0.2, -0.2, 0.0 ] },
            Vertex { position: [  0.2, -0.2, 0.0 ] },
        ]
    }

    fn vertex_buffer(display: &glium::Display) -> glium::VertexBuffer<Vertex> {
        glium::VertexBuffer::new(display, &Ball::vertices().as_slice()).unwrap()
    }
}
