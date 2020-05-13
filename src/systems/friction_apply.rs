use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};

use crate::components::{Velocity, Friction};

#[derive(SystemDesc)]
pub struct FrictionApplyingSystem;

impl<'s> System<'s> for FrictionApplyingSystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Friction>,
    );

    fn run(&mut self, (mut velocities, frictions): Self::SystemData) {
        for (velocity, friction) in (&mut velocities, &frictions).join() {
            *velocity = friction.apply(*velocity);
        }
    }
}
