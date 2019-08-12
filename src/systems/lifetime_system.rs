use crate::components::Lifetime;
use amethyst::{
    core::timing::Time,
    ecs::{Entities, Join, Read, System, WriteStorage},
};

pub struct LifetimeSystem;

impl<'s> System<'s> for LifetimeSystem {
    type SystemData = (WriteStorage<'s, Lifetime>, Read<'s, Time>, Entities<'s>);

    fn run(&mut self, (mut lifetimes, time, entities): Self::SystemData) {
        for (entity, lifetime) in (&entities, &mut lifetimes).join() {
            lifetime.timer += time.delta_seconds();
            if lifetime.timer >= lifetime.duration {
                entities.delete(entity).expect("Could not delete entity");
            }
        }
    }
}
