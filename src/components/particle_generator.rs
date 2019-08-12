use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct ParticleGenerator {
    pub sprite_indices: std::vec::Vec<i8>,
    pub interval: f32,
    /// Spawn interval in seconds
    pub timer: f32,
    pub active: bool
}

impl Component for ParticleGenerator {
    type Storage = DenseVecStorage<Self>;
}

impl ParticleGenerator {
    pub fn new(sprite_indices: std::vec::Vec<i8>, interval: f32) -> ParticleGenerator {
        ParticleGenerator {
            sprite_indices: sprite_indices,
            interval: interval,
            timer: 0.0,
            active: true,
        }
    }
}
