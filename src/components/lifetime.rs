use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Lifetime {
    pub duration: f32,
    pub timer: f32,
}

impl Component for Lifetime {
    type Storage = DenseVecStorage<Self>;
}

impl Lifetime {
    pub fn new(duration: f32) -> Lifetime {
        Lifetime {
            duration: duration,
            timer: 0.0,
        }
    }
}
