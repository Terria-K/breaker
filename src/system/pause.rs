use bevy_ecs::system::ResMut;
use macroquad::prelude::{is_key_pressed, KeyCode};

use crate::game::States;

pub fn system(mut state: ResMut<States>) {
    if !is_key_pressed(KeyCode::Escape) {
        return;
    }
    *state = if *state == States::Paused {
        States::Playing
    } else {
        States::Paused
    }
}
