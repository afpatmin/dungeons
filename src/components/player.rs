use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Player {}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Player {
    pub fn new() -> Player {
        Player {}
    }
}
