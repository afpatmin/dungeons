use crate::components::Player;
use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut transforms, players, input, time): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            let movement_y = input.axis_value("vertical");
            if let Some(mv_amount) = movement_y {
                if mv_amount != 0.0 {
                    transform.set_translation_y(transform.translation().y + mv_amount * 100.0 * time.delta_seconds());
                }
            }
            let movement_x = input.axis_value("horizontal");
            if let Some(mv_amount) = movement_x {
                if mv_amount != 0.0 {
                    transform.set_translation_x(transform.translation().x + mv_amount * 100.0 * time.delta_seconds());
                }
            }
        }
    }
}