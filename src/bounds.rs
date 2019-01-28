use crate::vector::Vector;

pub struct Bounds {
    pub position: Vector,
    pub dimensions: Vector,
}

impl Bounds {
    pub fn new(position: Vector, dimensions: Vector) -> Bounds {
        Bounds { position, dimensions }
    }

    pub fn intersects(&self, other: Bounds) -> bool {
        let intersectsX = self.position.x - self.dimensions.x / 2.0 <= other.position.x - other.dimensions.x / 2.0;
        let intersectsY = self.position.y - self.dimensions.y / 2.0 <= other.position.y - other.dimensions.y / 2.0;
        let intersectsZ = self.position.z - self.dimensions.z / 2.0 <= other.position.z - other.dimensions.z / 2.0;

        intersectsX || intersectsY || intersectsZ
    }

    pub fn contains(&self, other: Bounds) -> bool {
        let containsX = self.position.x - self.dimensions.x / 2.0 > other.position.x - other.dimensions.x / 2.0;
        let containsY = self.position.y - self.dimensions.y / 2.0 > other.position.y - other.dimensions.y / 2.0;
        let containsZ = self.position.z - self.dimensions.z / 2.0 > other.position.z - other.dimensions.z / 2.0;

        containsX && containsY && containsZ
    }
}