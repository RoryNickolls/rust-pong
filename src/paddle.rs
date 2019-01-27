use crate::render::{Render, Vertex};
use crate::update::Update;
use crate::rigidbody::Rigidbody;

pub struct Paddle {
    pub rigidbody: Rigidbody,
}

impl Paddle {
    pub fn new() -> Paddle {
        Paddle { rigidbody: Rigidbody::new() }
    }
}

impl Render for Paddle {
    fn model_matrix(&self) -> [[f32; 4]; 4] {
        self.rigidbody.transform.transform_matrix()
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

impl Update for Paddle {
    fn update(&mut self, delta_time: f32) {
        self.rigidbody.update(delta_time);
    }
}