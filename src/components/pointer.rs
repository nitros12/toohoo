use amethyst::ecs::{Component, FlaggedStorage};

#[derive(Debug, Copy, Clone, Component)]
#[storage(FlaggedStorage)]
pub struct Pointer;
