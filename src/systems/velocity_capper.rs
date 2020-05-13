use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};

use crate::components::{Velocity, CappedVelocity};

#[derive(SystemDesc)]
pub struct VelocityCapperSystem;

impl<'s> System<'s> for VelocityCapperSystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, CappedVelocity>,
    );

    fn run(&mut self, (mut velocities, caps): Self::SystemData) {
        for (velocity, cap) in (&mut velocities, &caps).join() {
            *velocity = cap.cap(*velocity);
        }
    }
}
