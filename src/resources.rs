use amethyst::{assets::Handle, renderer::SpriteSheet};

#[derive(Default)]
pub struct GameResource {
    pub sprite_sheet: Option<Handle<SpriteSheet>>,
}
