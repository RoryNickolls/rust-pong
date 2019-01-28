use crate::transform::Transform;
use crate::vector::Vector;
use crate::update::Update;
use crate::bounds::Bounds;
use crate::render::Vertex;

pub trait Rigidbody {
    fn on_collide<R: Rigidbody>(&mut self, other: Box<R>);
    fn bounds(&self) -> Bounds;
    fn transform(&self) -> &Transform;
    fn velocity(&self) -> &Vector;
}