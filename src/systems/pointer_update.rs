use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::{Pointer, Acceleration};

use crate::state::{ARENA_HEIGHT, ARENA_WIDTH, POINTER_DIAMETER};

#[derive(SystemDesc)]
pub struct PointerUpdateSystem;

impl<'s> System<'s> for PointerUpdateSystem {
    type SystemData = (
        WriteStorage<'s, Acceleration>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Pointer>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut accelerations, mut transforms, pointers, input): Self::SystemData) {
        for (acceleration, transform, _p) in (&mut accelerations, &mut transforms, &pointers).join() {
            let movement_x = input.axis_value("pointer_move_x");

            if let Some(mv_amount) = movement_x {
                let scaled_amount = 10.0 * mv_amount as f32;
                acceleration.x = scaled_amount;
            } else {
                acceleration.x = 0.0;
            }

            let movement_y = input.axis_value("pointer_move_y");

            if let Some(mv_amount) = movement_y {
                let scaled_amount = 10.0 * mv_amount as f32;
                acceleration.y = scaled_amount;
            } else {
                acceleration.y = 0.0;
            }


            let pointer_transl = *transform.translation();

            transform.set_translation_xyz(
                pointer_transl.x.min(ARENA_WIDTH - POINTER_DIAMETER * 0.5)
                                .max(POINTER_DIAMETER * 0.5),
                pointer_transl.y.min(ARENA_HEIGHT - POINTER_DIAMETER * 0.5)
                                .max(POINTER_DIAMETER * 0.5),
                pointer_transl.z,
            );

        }
    }
}
