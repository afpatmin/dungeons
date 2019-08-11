use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct ShipControl {}

impl Component for ShipControl {
    type Storage = DenseVecStorage<Self>;
}

impl ShipControl {
    pub fn new() -> ShipControl {
        ShipControl {}
    }
}
