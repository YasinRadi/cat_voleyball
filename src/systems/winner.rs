use amethyst::{
    ui::UiText,
    derive::SystemDesc,
    assets::AssetStorage,
    core::transform::Transform,
    audio::{output::Output, Source},
    ecs::prelude::{
        Join,
        Read,
        Write,
        World,
        System,
        ReadExpect,
        SystemData,
        WriteStorage,
    },
};

use std::ops::Deref;
use crate::audio::{play_score, Sounds};
use crate::catvolleyball::{Ball, ScoreBoard, ScoreText, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self, 
        (mut balls, mut locals, mut ui_text, 
        mut scores, score_text, storage, 
        sounds, audio_output): Self::SystemData,
    ) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            if ball_y <= ball.radius {
                // bottom frame
                if ball_x <= (ARENA_WIDTH / 2.0) {
                    scores.score_right = (scores.score_right + 1).min(999);

                    // Update score UI text
                    if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                        text.text = scores.score_right.to_string();
                    }
                } else {
                    scores.score_left = (scores.score_left + 1).min(999);

                    // Update score UI text
                    if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                        text.text = scores.score_left.to_string();
                    }
                }

                // reset ball position
                transform.set_translation_x(ARENA_WIDTH / 2.0);
                transform.set_translation_y(ARENA_HEIGHT / 2.0);

                // invert direction
                ball.velocity[0] = -ball.velocity[0];
                ball.velocity[1] = 0.0;

                play_score(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            }
        }
    }
}