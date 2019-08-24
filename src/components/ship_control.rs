use amethyst::{
    core::math,
    ecs::prelude::{Component, DenseVecStorage},
};

pub struct ShipControl {
    pub velocity: math::Vector3<f32>,
    pub acceleration: f32,
    pub max_speed: f32,
    pub drag: f32,
    pub turn_speed: f32,
}

impl Component for ShipControl {
    type Storage = DenseVecStorage<Self>;
}

impl ShipControl {
    pub fn new() -> ShipControl {
        ShipControl {
            velocity: math::Vector3::new(0.0, 0.0, 0.0),
            acceleration: 70.0,
            max_speed: 200.0,
            drag: 0.997,
            turn_speed: 3.14,
        }
    }
}
