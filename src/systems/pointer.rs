use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::Pointer;
use crate::state::{ARENA_HEIGHT, ARENA_WIDTH, POINTER_DIAMETER};

#[derive(SystemDesc)]
pub struct PointerSystem;

impl<'s> System<'s> for PointerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Pointer>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, pointers, input): Self::SystemData) {
        for (pointer, transform) in (&pointers, &mut transforms).join() {
            let movement_x = input.axis_value("pointer_move_x");

            if let Some(mv_amount) = movement_x {
                let scaled_amount = 1.2 * mv_amount as f32;
                let pointer_x = transform.translation().x;
                transform.set_translation_x(
                    (pointer_x + scaled_amount)
                        .min(ARENA_WIDTH - POINTER_DIAMETER * 0.5)
                        .max(ARENA_WIDTH * 0.5)
                );
            }

            let movement_y = input.axis_value("pointer_move_y");

            if let Some(mv_amount) = movement_y {
                let scaled_amount = 1.2 * mv_amount as f32;
                let pointer_y = transform.translation().y;
                transform.set_translation_y(
                    (pointer_y + scaled_amount)
                        .min(ARENA_HEIGHT - POINTER_DIAMETER * 0.5)
                        .max(ARENA_HEIGHT * 0.5)
                );
            }
        }
    }
}
