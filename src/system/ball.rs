use bevy_ecs::{
    prelude::Entity,
    query::With,
    system::{Commands, Query, Res, ResMut},
};

use crate::{
    component::{
        resource::{Score, Time},
        trigger::AddPlayer,
        Aabb, BallType, Position, Tags, Velocity,
    },
    game::{HEIGHT, WIDTH},
};

pub const BALL_NORMAL_SPEED: f32 = 250.0;

type BallQuery<'a> = (
    Entity,
    &'a mut Position,
    &'a mut Velocity,
    &'a Aabb,
    &'a Tags,
);

pub fn system(
    mut command: Commands,
    delta: Res<Time>,
    mut score: ResMut<Score>,
    mut query: Query<BallQuery, With<Velocity>>,
) {
    let score_speed = score.0 as f32 / 32.0;
    let score_speed = if score_speed >= 150.0 {
        150.0
    } else {
        score_speed
    };

    for (e, mut pos, mut vel, aabb, tags) in query.iter_mut() {
        pos.0.x += vel.0.x * delta.0 * (BALL_NORMAL_SPEED + score_speed);
        pos.0.y += vel.0.y * delta.0 * (BALL_NORMAL_SPEED + score_speed);

        if pos.0.x < 0f32 {
            vel.0.x = 1f32;
        }
        if pos.0.x > WIDTH - aabb.0.w {
            vel.0.x = -1f32;
        }
        if pos.0.y < 0f32 {
            vel.0.y = 1f32;
        }
        if pos.0.y > HEIGHT + aabb.0.h {
            command.entity(e).despawn();
            match tags {
                Tags::Ball(BallType::Original) => {
                    command.add(AddPlayer);
                }
                Tags::Ball(BallType::Clone) => {
                    score.0 -= 100;
                }
                _ => (),
            }
        }
    }
}
