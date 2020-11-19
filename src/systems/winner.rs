use amethyst::{
    core::{
        transform::Transform,
        SystemDesc,
    },
    derive::SystemDesc,
    ecs::prelude::{
        Join,
        World,
        Write,
        System,
        SystemData,
        WriteStorage,
    },
};

use crate::catvolleyball::{Ball, ScoreBoard, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Write<'s, ScoreBoard>,
    );

    fn run(&mut self, (mut balls, mut locals, mut scores): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            if ball_y <= ball.radius {
                // bottom frame
                if ball_x <= (ARENA_WIDTH / 2.0) {
                    scores.score_right = (scores.score_right + 1).min(999);
                } else {
                    scores.score_left = (scores.score_left + 1).min(999);
                }

                // reset ball position
                transform.set_translation_x(ARENA_WIDTH / 2.0);
                transform.set_translation_y(ARENA_HEIGHT / 2.0);

                // invert direction
                ball.velocity[0] = -ball.velocity[0];
                ball.velocity[1] = 0.0;
            }
        }
    }
}