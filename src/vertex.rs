extern crate nalgebra_glm as glm;

pub struct Vertex {
    pub position: glm::Vec3,
    pub color: glm::Vec3,
}

impl Vertex {
    pub fn new(position: glm::Vec3, color: glm::Vec3) -> Self {
        Self { position, color }
    }

    pub fn transform(&self, matrix: &glm::Mat4) -> Self {
        let transformed_position = matrix * glm::vec4(self.position.x, self.position.y, self.position.z, 1.0);
        Self {
            position: glm::vec3(transformed_position.x, transformed_position.y, transformed_position.z),
            color: self.color,
        }
    }
}
