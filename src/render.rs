use glium::implement_vertex;
use glium::VertexBuffer;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}
implement_vertex!(Vertex, position);


pub trait Render {
    fn model_matrix(&self) -> [[f32; 4]; 4];
    fn vertices() -> Vec<Vertex>;
    fn vertex_buffer(display: &glium::Display) -> glium::VertexBuffer<Vertex>;
}