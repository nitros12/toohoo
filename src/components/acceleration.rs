use amethyst::ecs::{Component, DenseVecStorage};

use nalgebra::base::{Vector3, Vector2};

#[derive(Debug, Copy, Clone, Component, Default)]
#[storage(DenseVecStorage)]
pub struct Acceleration {
    pub x: f32,
    pub y: f32,
}

impl Acceleration {
    pub fn as_vec3(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, 0.0)
    }

    pub fn as_vec2(&self) -> Vector2<f32> {
        Vector2::new(self.x, self.y)
    }
}
