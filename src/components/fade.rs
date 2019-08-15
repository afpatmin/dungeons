use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Fade {
    pub start_value: f32,
    pub end_value: f32,
    pub current_value: f32,
    pub duration: f32,
    pub step: f32,
}

impl Component for Fade {
    type Storage = DenseVecStorage<Self>;
}

impl Fade {
    pub fn new(start_value: f32, end_value: f32, duration: f32) -> Fade {
        Fade {
            start_value: start_value,
            end_value: end_value,
            current_value: start_value,
            duration: duration,
            step: (end_value - start_value) / duration,
        }
    }
}
