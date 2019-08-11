use crate::components::ShipControl;
use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub struct ShipControlSystem;

impl<'s> System<'s> for ShipControlSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, ShipControl>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut transforms, ship_controls, input, time): Self::SystemData) {
        for (ship_control, transform) in (&ship_controls, &mut transforms).join() {
            let movement_y = input.axis_value("acceleration");
            if let Some(mv_amount) = movement_y {
                if mv_amount != 0.0 {
                    transform.set_translation_y(transform.translation().y + mv_amount * 100.0 * time.delta_seconds());
                }
            }
            let movement_x = input.axis_value("turn");
            if let Some(mv_amount) = movement_x {
                if mv_amount != 0.0 {
                    transform.set_translation_x(transform.translation().x + mv_amount * 100.0 * time.delta_seconds());
                }
            }
        }
    }
}