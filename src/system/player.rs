use bevy_ecs::{
    query::Without,
    system::{Commands, Query, Res, ResMut},
};
use macroquad::prelude::{is_key_down, KeyCode};

use crate::{
    component::{
        resource::{FontResource, Time},
        trigger::AddGameOverText,
        Aabb, Lives, Position, Tags,
    },
    game::{States, WIDTH},
};

pub const PLAYER_SPEED: f32 = 900.0;

type PlayerQuery<'a> = (&'a mut Position, &'a Aabb, &'a Lives);

pub fn system(
    mut command: Commands,
    delta: Res<Time>,
    font: Res<FontResource>,
    mut state: ResMut<States>,
    mut query: Query<PlayerQuery, Without<Tags>>,
) {
    for (mut pos, aabb, lives) in query.iter_mut() {
        let axis = get_axis(KeyCode::Left, KeyCode::Right);

        pos.0.x += axis * delta.0 * PLAYER_SPEED;

        if pos.0.x < 0f32 {
            pos.0.x = 0f32;
        }
        if pos.0.x > WIDTH - aabb.0.w {
            pos.0.x = WIDTH - aabb.0.w;
        }
        if lives.0 <= 0 {
            *state = States::GameOver;
            command.add(AddGameOverText(font.0));
        }
    }
}

fn get_axis(key_code_a: KeyCode, key_code_b: KeyCode) -> f32 {
    match (is_key_down(key_code_a), is_key_down(key_code_b)) {
        (true, false) => -1f32,
        (false, true) => 1f32,
        _ => 0f32,
    }
}
