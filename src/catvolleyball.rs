use amethyst::{
    prelude::*,
    core::Transform,
    ecs::{Component, DenseVecStorage, Entity},
    assets::{AssetStorage, Handle, Loader},
    ui::{
        Anchor,
        UiText,
        LineMode,
        TtfFormat,
        UiTransform,
    },
    renderer::{
        Camera,
        Texture,
        ImageFormat,
        SpriteSheet,
        SpriteRender,
        SpriteSheetFormat
    },
};

use crate::audio::initialize_audio;

// Frame params
pub const ARENA_HEIGHT: f32 = 500.0;
pub const ARENA_WIDTH: f32 = 500.0;

// Player params
pub const PLAYER_WIDTH: f32 = 32.0;
pub const PLAYER_HEIGHT: f32 = 22.0;

// Ball params
pub const BALL_VELOCITY_X: f32 = 30.0;
pub const BALL_VELOCITY_Y: f32 = 0.0;
pub const BALL_RADIUS: f32 = 4.0;

// Audio params
pub const AUDIO_MUSIC: &'static [&'static str] = &[
    "./audio/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
    "./audio/Computer_Music_All-Stars_-_Albatross_v2.ogg",
];
pub const AUDIO_COLLISION: &'static str = "./audio/collision.ogg";
pub const AUDIO_SCORE: &'static str = "./audio/score.ogg";

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
    let sprite_render_left = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        // Cat will be the first sprite in list
        sprite_number: 0,
    };

    let sprite_render_right = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
    };

    world.create_entity()
        .with(sprite_render_left.clone())
        .with(Player::new(Side::Left))
        .with(left_transform)
        .build();

    world.create_entity()
        .with(sprite_render_right.clone())
        .with(Player::new(Side::Right))
        .with(right_transform)
        .build();
}

fn initialize_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(
        ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0
    );

    // Assign the sprite for the ball
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 2,
    };

    world.create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}

fn initialize_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>()
        .load(
            "font/square.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );  

        let p1_transform = UiTransform::new(
            "P1".to_string(),
            Anchor::TopMiddle,
            Anchor::Middle,
            -50.0,
            -50.0,
            1.0,
            200.0,
            50.0,
        );

        let p2_transform = UiTransform::new(
            "P2".to_string(),
            Anchor::TopMiddle,
            Anchor::Middle,
            50.0,
            -50.0,
            1.0,
            200.0,
            50.0,
        );

        let p1_score = world.create_entity()
            .with(p1_transform)
            .with(
                UiText::new(
                    font.clone(),
                    "0".to_string(),
                    [1.0, 1.0, 1.0, 1.0],
                    50.0,
                    LineMode::Single,
                    Anchor::Middle,
                )
            ).build();
        
        let p2_score = world.create_entity()
            .with(p2_transform)
            .with(
                UiText::new(
                    font.clone(),
                    "0".to_string(),
                    [1.0, 1.0, 1.0, 1.0],
                    50.0,
                    LineMode::Single,
                    Anchor::Middle,
                )
            ).build();

        world.insert(ScoreText {
            p1_score,
            p2_score,
        });
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

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

// Game state
pub struct CatVolleyball;

impl SimpleState for CatVolleyball {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        initialize_camera(world);
        world.register::<Player>();
        initialize_ball(world, sprite_sheet_handle.clone());
        initialize_players(world, sprite_sheet_handle);
        initialize_scoreboard(world);
        initialize_audio(world);
    }
}