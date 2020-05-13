use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};

use crate::components::{Velocity, Acceleration};

#[derive(SystemDesc)]
pub struct VelocityUpdateSystem;

impl<'s> System<'s> for VelocityUpdateSystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Acceleration>,
    );

    fn run(&mut self, (mut velocities, accelerations): Self::SystemData) {
        for (velocity, acceleration) in (&mut velocities, &accelerations).join() {
            *velocity += *acceleration;
        }
    }
}
