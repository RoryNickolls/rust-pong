use crate::rigidbody::Rigidbody;
use crate::render::{Render, Vertex};
use crate::update::Update;
use crate::vector::Vector;

pub struct Ball {
    pub rigidbody: Rigidbody,
    pub in_play: bool,
}

impl Ball {
    pub fn new() -> Ball {
        let mut rigidbody = Rigidbody::new();
        rigidbody.transform.scale = Vector::new(0.1, 0.1, 0.1);

        Ball { rigidbody: rigidbody, in_play: true, }
    }
}

impl Update for Ball {
    fn update(&mut self, delta_time: f32) {
        if self.in_play {
            self.rigidbody.update(delta_time);
        }

        if self.rigidbody.transform.position.y <= -1.0 {
            self.rigidbody.transform.position.y = 1.0;
        }
    }
}

impl Render for Ball {
    fn model_matrix(&self) -> [[f32; 4]; 4] {
        self.rigidbody.transform.transform_matrix()
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
