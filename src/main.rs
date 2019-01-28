use glium::uniform;
use glium::glutin;
use glium::Program;

use time::PreciseTime;

pub mod rigidbody;
use rigidbody::Rigidbody;
pub mod ball;
use ball::Ball;
pub mod render;
use render::Vertex;
use render::Render;
pub mod transform;
pub mod vector;
use vector::Vector;
pub mod update;
use update::Update;
pub mod paddle;
use paddle::Paddle;

pub mod bounds;

struct GameLoop {
    display: glium::Display,
    elapsed_time: f32,
    delta_time: f32,
    running: bool,
}

struct RigidbodySystem<R: Rigidbody> {
    actors: Vec<Box<R>>,
}

impl<R> Update for RigidbodySystem<R>
    where R: Rigidbody + Render + Update {
        
    fn update(&mut self, delta_time: f32) {
        for actor in self.actors {
            let a = *actor;
            a.update(delta_time);
        }
    }
}

impl GameLoop {
    fn new(display: glium::Display) -> GameLoop {
        let mut player_paddle = Paddle::new();
        player_paddle.transform.position = Vector::new(-0.95, 1.0, 0.0);
        player_paddle.transform.scale = Vector::new(0.1, 0.1, 0.1);

        let mut enemy_paddle = Paddle::new();
        enemy_paddle.rigidbody.transform.position = Vector::new(0.95, 0.0, 0.0);
        enemy_paddle.rigidbody.transform.scale = Vector::new(0.1, 0.1, 0.1);

        let mut ball = Ball::new();
        ball.rigidbody.transform.position = Vector::zero();
        ball.rigidbody.transform.scale = Vector::new(0.1, 0.1, 0.1);
        ball.rigidbody.velocity = Vector::new(-1.0, 0.0, 0.0);

        GameLoop { display: display, elapsed_time: 0.0, delta_time: 0.0, running: true, player_paddle: player_paddle, enemy_paddle: enemy_paddle, ball: ball }
    }

    pub fn render(&self, program: &Program) {
        use glium::Surface;


        let mut target = self.display.draw();
        let view = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = width as f32 / height as f32;
            [
                [ 1.0, 0.0, 0.0, 0.0 ],
                [ 0.0, aspect_ratio, 0.0, 0.0 ],
                [ 0.0, 0.0, 0.0, 0.0 ],
                [ 0.0, 0.0, 0.0, 1.0f32 ],
            ]
        };
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target.draw(&Paddle::vertex_buffer(&self.display), glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                    program, &uniform! { model: self.player_paddle.model_matrix() }, &glium::DrawParameters::default()).unwrap();

        target.draw(&Paddle::vertex_buffer(&self.display), glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                    program, &uniform! { model: self.enemy_paddle.model_matrix() }, &glium::DrawParameters::default()).unwrap();

        target.draw(&Ball::vertex_buffer(&self.display), glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                    program, &uniform! { model: self.ball.rigidbody.transform.transform_matrix(), view: view }, &glium::DrawParameters::default()).unwrap();

        target.finish().unwrap();
    } 

    pub fn update(&mut self, mut events_loop: glutin::EventsLoop, program: glium::Program) {
        while self.running {
            let last_frame = PreciseTime::now();
            self.render(&program);

            self.delta_time = last_frame.to(PreciseTime::now()).num_nanoseconds().unwrap() as f32 / 1000000000.0;
            self.elapsed_time += self.delta_time;

            self.player_paddle.update(self.delta_time);
            self.ball.update(self.delta_time);
            self.enemy_paddle.update(self.delta_time);

            events_loop.poll_events(|ev| {
                self.handle_event(ev);
            });
        }
    }

    pub fn handle_event(&mut self, event: glutin::Event) {
        match event {
            glutin::Event::WindowEvent { event: window_evt, .. } => {
                match window_evt {
                    glutin::WindowEvent::CloseRequested => self.kill(),
                    glutin::WindowEvent::KeyboardInput { input: input_evt, .. } => {
                        let input = (input_evt.virtual_keycode, input_evt.state);
                        match input {
                            (Some(glutin::VirtualKeyCode::Up), glutin::ElementState::Pressed) => self.player_paddle.rigidbody.velocity = Vector::up(),
                            (Some(glutin::VirtualKeyCode::Down), glutin::ElementState::Pressed) => self.player_paddle.rigidbody.velocity = -Vector::up(),
                            (_, glutin::ElementState::Released) => self.player_paddle.rigidbody.velocity = Vector::zero(),
                            _ => (),
                        };
                    }
                    _ => (),
                };
            },
            _ => (),
        }
    }

    pub fn kill(&mut self) {
        self.running = false;
    }
}

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

    let mut game_loop = GameLoop::new(display);
    game_loop.update(events_loop, program);
}