use glium::implement_vertex;
use glium::VertexBuffer;
use glium::{Frame, Display, Program};
use glium;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}
implement_vertex!(Vertex, position);

pub struct Renderable {
    model_matrix: [[f32; 4]; 4],
    vertices: Vec<Vertex>,
    vertex_buffer: Option<VertexBuffer<Vertex>>,
}

impl Renderable {

    pub fn new(model_matrix: [[f32; 4]; 4], vertices: Vec<Vertex>) -> Renderable {
        Renderable { model_matrix: model_matrix, vertices: vertices, vertex_buffer: None }
    }

    pub fn vertex_buffer(&self) -> Option<&VertexBuffer<Vertex>> {
        if let Some(vb) = &self.vertex_buffer {
            return Some(&vb)
        }

        None
    }

    pub fn model_matrix(&self) -> [[f32; 4]; 4] {
        self.model_matrix
    }

    pub fn set_model_matrix(&mut self, matrix: [[f32; 4]; 4]) {
        self.model_matrix = matrix;
    }

    pub fn generate_vertex_buffer(&mut self, display: &glium::Display) {
        self.vertex_buffer = Some(VertexBuffer::new(display, &self.vertices.as_slice()).unwrap());
    }
}

pub struct GlobalRenderParameters {
    display: glium::Display,
    program: glium::Program,
}

impl GlobalRenderParameters {
    pub fn new(display: glium::Display, program: glium::Program) -> GlobalRenderParameters {
        GlobalRenderParameters { display: display, program: program }
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}

pub struct FrameRenderParameters {
    frame: glium::Frame,
    view_matrix: [[f32; 4]; 4],
}

impl FrameRenderParameters {
    pub fn new(frame: glium::Frame, view_matrix: [[f32; 4]; 4]) -> FrameRenderParameters {
        FrameRenderParameters { frame: frame, view_matrix: view_matrix }
    }

    pub fn frame(&mut self) -> &mut Frame {
        &mut self.frame
    }

    pub fn finish_frame(self) {
        self.frame.finish().unwrap();
    }

    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        self.view_matrix
    }
}