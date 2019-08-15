use crate::components::Fade;
use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, System, WriteStorage},
    renderer::resources::Tint,
};

pub struct FadeSystem;

impl<'s> System<'s> for FadeSystem {
    type SystemData = (
        WriteStorage<'s, Fade>,
        WriteStorage<'s, Tint>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut fades, mut tints, time): Self::SystemData) {
        for (fade, tint) in (&mut fades, &mut tints).join() {
            fade.current_value += fade.step * time.delta_seconds();
            if (fade.step > 0.0 && fade.current_value > fade.end_value)
                || (fade.step < 0.0 && fade.current_value < fade.end_value)
            {
                fade.current_value = fade.end_value;
            }
            tint.0.alpha = fade.current_value;
        }
    }
}