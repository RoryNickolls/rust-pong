use crate::glutin;
use crate::ecs::EntityComponentSystem;
use crate::paddle::Paddle;
use crate::ball::Ball;
use crate::ecs::Entity;
use crate::vector::Vector;
use crate::render::{GlobalRenderParameters};
use time::PreciseTime;

pub struct GameLoop {
    elapsed_time: f32,
    delta_time: f32,
    running: bool,
    entity_component_system: EntityComponentSystem,
}

impl GameLoop {
    pub fn new(display: glium::Display, program: glium::Program) -> GameLoop {
        let mut player_paddle = Paddle::new(true);
        player_paddle.rigidbody().unwrap().transform_mut().position = Vector::new(-0.95, 1.0, 0.0);
        player_paddle.rigidbody().unwrap().transform_mut().scale = Vector::new(0.1, 0.1, 0.1);

        let mut enemy_paddle = Paddle::new(false);
        enemy_paddle.rigidbody().unwrap().transform_mut().position = Vector::new(0.95, 0.0, 0.0);
        enemy_paddle.rigidbody().unwrap().transform_mut().scale = Vector::new(0.1, 0.1, 0.1);

        let mut ball = Ball::new();
        ball.rigidbody().unwrap().transform_mut().position = Vector::zero();
        ball.rigidbody().unwrap().transform_mut().scale = Vector::new(0.1, 0.1, 0.1);
        ball.rigidbody().unwrap().velocity_mut().x = -1.0;

        let mut entity_component_system = EntityComponentSystem::new(GlobalRenderParameters::new(display, program));
        entity_component_system.add_entity(Box::new(player_paddle));
        entity_component_system.add_entity(Box::new(enemy_paddle));
        entity_component_system.add_entity(Box::new(ball));
        GameLoop { elapsed_time: 0.0, delta_time: 0.0, running: true, entity_component_system: entity_component_system }
    }

    pub fn update(&mut self, mut events_loop: glutin::EventsLoop) {
        let mut events: Vec<glutin::Event> = Vec::new();
        while self.running {
            let last_frame = PreciseTime::now();

            events = Vec::new();
            events_loop.poll_events(|ev| {
                self.handle_event(&ev);
                events.push(ev);
            });
            self.entity_component_system.update(self.delta_time, events);
            
            self.delta_time = last_frame.to(PreciseTime::now()).num_nanoseconds().unwrap() as f32 / 1000000000.0;
            self.elapsed_time += self.delta_time;  
        }
    }

    pub fn handle_event(&mut self, event: &glutin::Event) {
        match event {
            glutin::Event::WindowEvent { event: window_evt, .. } => {
                match window_evt {
                    glutin::WindowEvent::CloseRequested => self.kill(),
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