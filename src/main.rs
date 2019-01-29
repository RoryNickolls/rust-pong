use glium::glutin;
use glium::Program;

pub mod rigidbody;
pub mod ball;
pub mod render;
pub mod transform;
pub mod vector;
pub mod paddle;
pub mod bounds;
pub mod ecs;

pub mod game_loop;
use game_loop::GameLoop;


fn main() {
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(glutin::dpi::LogicalSize::new(640.0, 640.0))
        .with_title("Pong");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let vertex_shader = r#"
        #version 140
        
        in vec3 position;

        uniform mat4 model;
        uniform mat4 view;
        
        void main() {
            gl_Position = view * model * vec4(position, 1.0);
        }
    "#;

    let fragment_shader = r#"
        #version 140

        out vec4 color;
        
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, &vertex_shader, &fragment_shader, None).unwrap();

    let mut game_loop = GameLoop::new(display, program);
    game_loop.update(events_loop);
}