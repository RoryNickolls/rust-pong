use crate::render::{Render, Vertex};
use crate::vector::Vector;
use crate::bounds::Bounds;
use crate::transform::Transform;
use crate::rigidbody::Rigidbody;

pub struct Paddle { 
    pub transform: Transform,
    pub velocity: Vector,
}

impl Paddle {
    pub fn new() -> Paddle {
        Paddle { transform: Transform::new(), velocity: Vector::zero(), }
    }
}

impl Rigidbody for Paddle {
    fn on_collide<R: Rigidbody>(&mut self, other: Box<R>) {

    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn velocity(&self) -> &Vector {
        &self.velocity
    }

    fn bounds(&self) -> Bounds {
        Bounds::new(self.transform().position, Vector::zero())
    }
}

impl Render for Paddle {
    fn model_matrix(&self) -> [[f32; 4]; 4] {
        self.transform().transform_matrix()
    }

    fn vertices() -> Vec<Vertex> {
        vec!
        [
            Vertex { position: [   -0.3,    1.0,    0.0 ] },
            Vertex { position: [    0.3,    1.0,    0.0 ] },
            Vertex { position: [   -0.3,   -1.0,    0.0 ] },
            Vertex { position: [    0.3,   -1.0,    0.0 ] },
        ]
    }

    fn vertex_buffer(display: &glium::Display) -> glium::VertexBuffer<Vertex> {
        glium::VertexBuffer::new(display, &Paddle::vertices().as_slice()).unwrap()
    }
}