use bevy_ecs::prelude::Entity;
use macroquad::{prelude::Rect, text::Font};

pub struct Score(pub i32);
pub struct Time(pub f32);

pub struct ColliderResource(pub Vec<(Entity, Rect)>);
pub struct FontResource(pub Font);
