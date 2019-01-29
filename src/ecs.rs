use crate::rigidbody::Rigidbody;
use crate::render::{GlobalRenderParameters, FrameRenderParameters, Renderable};
use glium::{uniform, Surface, Frame, Display};
use glium::glutin;

pub trait Entity {
    fn rigidbody(&mut self) -> Option<&mut Rigidbody>;
    fn renderable(&mut self) -> Option<&mut Renderable>;
    fn process_input_event(&mut self, event: &glutin::Event);
}

pub struct EntityComponentSystem {
    global_render_parameters: GlobalRenderParameters,
    entities: Vec<Box<dyn Entity>>,
}

impl EntityComponentSystem {
    pub fn new(global_render_parameters: GlobalRenderParameters) -> EntityComponentSystem {
        EntityComponentSystem { global_render_parameters: global_render_parameters, entities: vec![] }
    }

    pub fn update(&mut self, delta_time: f32, events: Vec<glutin::Event>) {
        let mut target = self.global_render_parameters.display().draw();
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

        let mut frame_render_parameters = FrameRenderParameters::new(target, view); 
        for entity in self.entities.iter_mut() {
            if let Some(renderable) = entity.renderable() {
                EntityComponentSystem::process_renderable(&self.global_render_parameters, &mut frame_render_parameters, renderable);
            }

            if let Some(rigidbody) = entity.rigidbody() {
                EntityComponentSystem::process_rigidbody(delta_time, rigidbody);
            }

            for event in &events {
                entity.process_input_event(event);
            }
        }
        frame_render_parameters.finish_frame();
    }

    pub fn add_entity(&mut self, entity: Box<Entity>) {
        self.entities.push(entity);
    }

    fn process_rigidbody(delta_time: f32, rigidbody: &mut Rigidbody) {
        let new_position = rigidbody.transform().position + rigidbody.velocity() * delta_time;
        rigidbody.transform_mut().position = new_position;
    }

    fn process_renderable(global_render_parameters: &GlobalRenderParameters, frame_render_parameters: &mut FrameRenderParameters, renderable: &mut Renderable) {
        let vb = renderable.vertex_buffer();
        if let None = vb {
            renderable.generate_vertex_buffer(global_render_parameters.display());
        }

        let view = frame_render_parameters.view_matrix();
        frame_render_parameters.frame().draw(renderable.vertex_buffer().unwrap(), glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
            &global_render_parameters.program(), &uniform! { model: renderable.model_matrix(), view: view }, &glium::DrawParameters::default()).unwrap();
    }
}