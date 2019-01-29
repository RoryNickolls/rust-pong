use crate::rigidbody::Rigidbody;
use crate::render::{Vertex, Renderable};
use crate::transform::Transform;
use crate::vector::Vector;
use crate::bounds::Bounds;
use crate::ecs::Entity;

pub struct Ball {
    rigidbody: Rigidbody,
    renderable: Renderable,
    in_play: bool,
}

impl Ball {
    pub fn new() -> Ball {
        let rigidbody = Rigidbody::new(Transform::new(), Vector::zero(), Bounds::new(Vector::zero(), Vector::zero()));
        let vertices = vec![
            Vertex { position: [ -0.2,  0.2, 0.0 ] },
            Vertex { position: [  0.2,  0.2, 0.0 ] },
            Vertex { position: [ -0.2, -0.2, 0.0 ] },
            Vertex { position: [  0.2, -0.2, 0.0 ] },
        ];
        let renderable = Renderable::new(rigidbody.transform().transform_matrix(), vertices);
        Ball { rigidbody: rigidbody, renderable: renderable, in_play: true, }
    }
}

impl Entity for Ball {
    fn rigidbody(&mut self) -> Option<&mut Rigidbody> {
        Some(&mut self.rigidbody)
    }

    fn renderable(&mut self) -> Option<&mut Renderable> {
        self.renderable.set_model_matrix(self.rigidbody.transform().transform_matrix());
        Some(&mut self.renderable)
    }
}
