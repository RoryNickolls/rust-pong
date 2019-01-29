use crate::render::{Vertex, Renderable};
use crate::vector::Vector;
use crate::bounds::Bounds;
use crate::transform::Transform;
use crate::rigidbody::Rigidbody;
use crate::ecs::Entity;
use glium::glutin;

pub struct Paddle { 
    rigidbody: Rigidbody,
    renderable: Renderable,
    is_player: bool,
}

impl Paddle {
    pub fn new(is_player: bool) -> Paddle {
        let rigidbody = Rigidbody::new(Transform::new(), Vector::zero(), Bounds::new(Vector::zero(), Vector::zero()));
        let vertices = vec![
            Vertex { position: [   -0.3,    1.0,    0.0 ] },
            Vertex { position: [    0.3,    1.0,    0.0 ] },
            Vertex { position: [   -0.3,   -1.0,    0.0 ] },
            Vertex { position: [    0.3,   -1.0,    0.0 ] },
        ];
        let renderable = Renderable::new(rigidbody.transform().transform_matrix(), vertices);
        Paddle { rigidbody: rigidbody, renderable: renderable, is_player: is_player }
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

    fn process_input_event(&mut self, event: &glutin::Event) {
        if self.is_player {
            match event {
                glutin::Event::WindowEvent { event: window_evt, .. } => {
                    match window_evt {
                        glutin::WindowEvent::KeyboardInput { input: input_evt, .. } => {
                         let input = (input_evt.virtual_keycode, input_evt.state);
                            match input {
                                (Some(glutin::VirtualKeyCode::Up), glutin::ElementState::Pressed) => *self.rigidbody.velocity_mut() = Vector::up(),
                                (Some(glutin::VirtualKeyCode::Down), glutin::ElementState::Pressed) => *self.rigidbody.velocity_mut() = -Vector::up(),
                                (_, glutin::ElementState::Released) => *self.rigidbody.velocity_mut() = Vector::zero(),
                                _ => (),
                            };
                        }
                        _ => (),
                    };
                },
                _ => (),
            }
        }
    }
}