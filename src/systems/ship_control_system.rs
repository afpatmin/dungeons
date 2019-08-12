use crate::components::{ParticleGenerator, ShipControl};
use amethyst::{
    core::{math, timing::Time, Transform},
    ecs::{Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub struct ShipControlSystem;

impl<'s> System<'s> for ShipControlSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, ShipControl>,
        WriteStorage<'s, ParticleGenerator>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut transforms, mut ship_controls, mut particle_generators, input, time): Self::SystemData,
    ) {
        for (ship_control, transform, particle_generator) in (
            &mut ship_controls,
            &mut transforms,
            &mut particle_generators,
        )
            .join()
        {
            let dt = time.delta_seconds();
            let acceleration = input.axis_value("acceleration");
            if let Some(thrust) = acceleration {
                if thrust > 0.0 {
                    let direction = transform
                        .rotation()
                        .transform_vector(&math::Vector3::new(0.0, 1.0, 0.0));
                    ship_control.velocity += direction * ship_control.acceleration * dt;

                    if ship_control.velocity.norm() > ship_control.max_speed {
                        ship_control.velocity =
                            ship_control.velocity.normalize() * ship_control.max_speed;
                    }
                    particle_generator.active = true;
                } else {
                    particle_generator.active = false;
                }
            }
            let turn = input.axis_value("turn");
            if let Some(direction) = turn {
                if direction != 0.0 {
                    transform.append_rotation_z_axis(ship_control.turn_speed * direction * dt);
                }
            }
            ship_control.velocity *= ship_control.drag;
            transform.prepend_translation(ship_control.velocity * dt);
        }
    }
}
