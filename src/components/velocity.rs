use amethyst::ecs::{Component, DenseVecStorage};

use nalgebra::base::{Vector3, Vector2};

use super::Acceleration;

#[derive(Debug, Copy, Clone, Component, Default)]
#[storage(DenseVecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn as_vec3(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, 0.0)
    }

    pub fn as_vec2(&self) -> Vector2<f32> {
        Vector2::new(self.x, self.y)
    }
}

impl From<Vector2<f32>> for Velocity {
    fn from(v: Vector2<f32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl std::ops::Add<Acceleration> for Velocity {
    type Output = Self;

    fn add(self, rhs: Acceleration) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign<Acceleration> for Velocity {
    fn add_assign(&mut self, rhs: Acceleration) {
        *self = *self + rhs;
    }
}
