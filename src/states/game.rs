pub const CAMERA_WIDTH: f32 = 2500.0;
pub const CAMERA_HEIGHT: f32 = 2500.0;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::components::{ParticleGenerator, ShipControl};

use crate::resources::GameResource;

pub struct Game;

impl Game {}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet = load_sprite_sheet(world);
        let game_resource = GameResource {
            sprite_sheet: Some(sprite_sheet.clone()),
        };
        world.add_resource(game_resource);

        init_camera(world);
        init_ship(world, sprite_sheet.clone());
    }
}

fn init_ship(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, 0.0);
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(ParticleGenerator::new(vec![0], 0.01))
        .with(ShipControl::new())
        .with(sprite_render.clone())
        .with(transform)
        .build();
}

fn init_camera(world: &mut World) {
    // (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(CAMERA_WIDTH, CAMERA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        let image_format = ImageFormat::default();        
        loader.load(
            "textures/pong_spritesheet.png",
            image_format,
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
