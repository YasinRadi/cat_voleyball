use amethyst::{
    core::{
        SystemDesc,
        timing::Time,
        transform::Transform,
    },
    derive::SystemDesc,
    ecs::prelude::{
        Join,
        Read,
        World,
        System,
        SystemData,
        WriteStorage,
    },
};

use crate::catvolleyball::Ball;

#[derive(SystemDesc)]
pub struct MoveBallSystem;

pub const GRAVITY_ACCELERATION: f32 = -40.0;

impl<'s> System<'s> for MoveBallSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run (&mut self, (mut balls, mut locals, time): Self::SystemData) {
        // Move ball according to gravity values
        for (ball, local) in (&mut balls, &mut locals).join() {
            local.prepend_translation_x(
                ball.velocity[0] * time.delta_seconds()
            );

            local.prepend_translation_y(
                (
                    ball.velocity[1] +
                    time.delta_seconds() *
                    GRAVITY_ACCELERATION / 2.0
                ) * time.delta_seconds(),
            );

            ball.velocity[1] = ball.velocity[1] + time.delta_seconds() * GRAVITY_ACCELERATION;
        }
    }
}