use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug, Copy, Clone, Component)]
#[storage(DenseVecStorage)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}
