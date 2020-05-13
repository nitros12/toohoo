use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};

use crate::components::Velocity;

#[derive(SystemDesc)]
pub struct PositionUpdateSystem;

impl<'s> System<'s> for PositionUpdateSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
    );

    fn run(&mut self, (mut transforms, velocities): Self::SystemData) {
        for (transform, velocity) in (&mut transforms, &velocities).join() {
            transform.append_translation(velocity.as_vec3());
        }
    }
}
