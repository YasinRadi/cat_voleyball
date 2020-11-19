extern crate rand;

use rand::Rng;
use amethyst::{
    core::{transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{
        Join,
        System,
        ReadStorage,
        WriteStorage,
    },
};

use crate::catvolleyball::{Ball, Player, Side, ARENA_HEIGHT, ARENA_WIDTH};


fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, players, transforms): Self::SystemData) {
        // Handle ball collisions and velocity check
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            // Bounce on frame collision
            if ball_y <= ball.radius && ball.velocity[1] < 0.0 {
                ball.velocity[1] = -ball.velocity[1];
            } else if ball_y >= (ARENA_HEIGHT - ball.radius) && ball.velocity[1] > 0.0 {
                ball.velocity[1] = -ball.velocity[1];
            } else if ball_x <= (ball.radius) && ball.velocity[0] < 0.0 {
                ball.velocity[0] = -ball.velocity[0];
            } else if ball_x >= (ARENA_WIDTH - ball.radius) && ball.velocity[0] > 0.0 {
                ball.velocity[0] = -ball.velocity[0];
            }

            for (player, player_transform) in (&players, &transforms).join() {
                let player_x = player_transform.translation().x - (player.width * 0.5);
                let player_y = player_transform.translation().y - (player.height * 0.5);
                
                if point_in_rect(
                    ball_x, 
                    ball_y, 
                    player_x - ball.radius, 
                    player_y - ball.radius, 
                    player_x + player.width + ball.radius, 
                    player_y + player.height + ball.radius,
                ) {
                    if ball.velocity[1] < 0.0 {
                        // Only bounce when ball is falling
                        ball.velocity[1] = -ball.velocity[1];

                        let mut rng = rand::thread_rng();
                        match player.side {
                            Side::Left => {
                                ball.velocity[0] = ball.velocity[0].abs() * 
                                    rng.gen_range(0.6, 1.4);
                            }

                            Side::Right => {
                                ball.velocity[0] = -ball.velocity[0].abs() *
                                    rng.gen_range(0.6, 1.4);
                            }
                        }
                    }
                }
            }
        }
    } 
}