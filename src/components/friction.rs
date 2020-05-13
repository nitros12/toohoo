use amethyst::ecs::{Component, DenseVecStorage};

use super::Velocity;

#[derive(Debug, Copy, Clone, Component)]
#[storage(DenseVecStorage)]
pub struct Friction {
    pub coefficient: f32,
}

impl Friction {
    pub fn new(coefficient: f32) -> Self {
        Friction {
            coefficient,
        }
    }

    pub fn apply(&self, velo: Velocity) -> Velocity {
        velo.as_vec2().scale(self.coefficient).into()
    }
}
