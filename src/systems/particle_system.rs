use crate::components::{Fade, Lifetime, ParticleGenerator};
use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Entities, Join, Read, System, WriteStorage},
    renderer::{palette::Srgba, resources::Tint, SpriteRender, Transparent},
};

use crate::resources::GameResource;

pub struct ParticleSystem;

impl<'s> System<'s> for ParticleSystem {
    type SystemData = (
        Read<'s, GameResource>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, ParticleGenerator>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Lifetime>,
        WriteStorage<'s, Fade>,
        WriteStorage<'s, Tint>,
        WriteStorage<'s, Transparent>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            game_resource,
            mut transforms,
            mut particle_generators,
            mut sprite_renders,
            mut lifetimes,
            mut fades,
            mut tints,
            mut transparents,
            time,
            entities,
        ): Self::SystemData,
    ) {
        for (entity, particle_generator) in (&entities, &mut particle_generators).join() {
            particle_generator.timer += time.delta_seconds();
            if particle_generator.active == true
                && particle_generator.timer >= particle_generator.interval
            {
                if let Some(transform) = transforms.get(entity) {
                    if let Some(sprite_sheet) = &game_resource.sprite_sheet {
                        let sprite_render = SpriteRender {
                            sprite_sheet: sprite_sheet.clone(),
                            sprite_number: 1,
                        };

                        entities
                            .build_entity()
                            .with(transform.clone(), &mut transforms)
                            .with(sprite_render, &mut sprite_renders)
                            .with(Lifetime::new(2.5), &mut lifetimes)
                            .with(Fade::new(1.0, 0.0, 1.5), &mut fades)
                            .with(Transparent, &mut transparents)
                            .with(Tint(Srgba::new(1.0, 1.0, 1.0, 1.0)), &mut tints)
                            .build();
                    }
                }

                particle_generator.timer = 0.0;
            }
        }
    }
}
