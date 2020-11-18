use amethyst::{
    prelude::*,
    core::Transform,
    ecs::{Component, DenseVecStorage},
    assets::{AssetStorage, Handle, Loader},
    renderer::{
        Camera,
        Texture,
        ImageFormat,
        SpriteSheet,
        SpriteRender,
        SpriteSheetFormat
    }
};

pub const ARENA_HEIGHT: f32 = 500.0;
pub const ARENA_WIDTH: f32 = 500.0;
pub const PLAYER_WIDTH: f32 = 32.0;
pub const PLAYER_HEIGHT: f32 = 22.0;

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store
    )
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        ARENA_WIDTH * 0.5,
        ARENA_HEIGHT * 0.5,
        1.0
    );

    world.create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialize_players(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();
    let y = PLAYER_HEIGHT / 2.0;

    // Set up both sides transforms
    left_transform.set_translation_xyz(PLAYER_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(
            ARENA_WIDTH - PLAYER_WIDTH * 0.5, 
            y, 0.0
    );

    // Sprite render
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        // Cat will be the first sprite in list
        sprite_number: 0
    };

    world.create_entity()
        .with(sprite_render.clone())
        .with(Player::new(Side::Left))
        .with(left_transform)
        .build();

    world.create_entity()
        .with(sprite_render.clone())
        .with(Player::new(Side::Right))
        .with(right_transform)
        .build();
}


#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right
}

// Player entity
pub struct Player {
    pub side: Side,
    pub width: f32,
    pub height: f32
}

impl Player {
    fn new(side: Side) -> Player {
        Player {
            side,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

// Game state
pub struct CatVolleyball;

impl SimpleState for CatVolleyball {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        initialize_camera(world);
        world.register::<Player>();
        initialize_players(world, sprite_sheet_handle);
    }
}