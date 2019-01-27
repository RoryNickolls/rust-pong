use crate::vector::Vector;

#[derive(Clone)]
pub struct Transform {
    pub position: Vector,
    pub rotation: Vector,
    pub scale: Vector,
}

impl Transform {
    pub fn new() -> Transform {
        Transform { position: Vector::zero(), rotation: Vector::zero(), scale: Vector::one() }
    }

    pub fn transform_matrix(&self) -> [[f32; 4]; 4] { 
        [
         [self.scale.x, 0.0, 0.0, 0.0],
         [0.0, self.scale.y, 0.0, 0.0],
         [0.0, 0.0, self.scale.z, 0.0],
         [self.position.x, self.position.y, self.position.z, 1.0],
        ]
    }
}