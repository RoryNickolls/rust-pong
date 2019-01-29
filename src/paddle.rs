use crate::render::{Vertex, Renderable};
use crate::vector::Vector;
use crate::bounds::Bounds;
use crate::transform::Transform;
use crate::rigidbody::Rigidbody;
use crate::ecs::Entity;

pub struct Paddle { 
    rigidbody: Rigidbody,
    renderable: Renderable,
}

impl Paddle {
    pub fn new() -> Paddle {
        let rigidbody = Rigidbody::new(Transform::new(), Vector::zero(), Bounds::new(Vector::zero(), Vector::zero()));
        let vertices = vec![
            Vertex { position: [   -0.3,    1.0,    0.0 ] },
            Vertex { position: [    0.3,    1.0,    0.0 ] },
            Vertex { position: [   -0.3,   -1.0,    0.0 ] },
            Vertex { position: [    0.3,   -1.0,    0.0 ] },
        ];
        let renderable = Renderable::new(rigidbody.transform().transform_matrix(), vertices);
        Paddle { rigidbody: rigidbody, renderable: renderable }
    }
}

impl Entity for Paddle {
    fn rigidbody(&mut self) -> Option<&mut Rigidbody> {
        Some(&mut self.rigidbody)
    }
    
    fn renderable(&mut self) -> Option<&mut Renderable> {
        self.renderable.set_model_matrix(self.rigidbody.transform().transform_matrix());
        Some(&mut self.renderable)
    }
}