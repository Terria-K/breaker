use bevy_ecs::system::{Query, Res};
use macroquad::{
    prelude::{set_camera, set_default_camera, BLACK, DARKGRAY},
    texture::{draw_texture_ex, DrawTextureParams},
    window::clear_background,
};

use crate::component::{ColorComponent, Position, Resolution, TextureComponent};

type DrawQuery<'a> = (&'a Position, &'a TextureComponent, &'a ColorComponent);

pub fn system(query: Query<DrawQuery>) {
    for (pos, tex, color) in query.iter() {
        let texture_params = DrawTextureParams {
            source: Some(tex.source),
            ..Default::default()
        };
        draw_texture_ex(tex.texture, pos.0.x, pos.0.y, color.0, texture_params);
    }
}

pub fn pre_system(resolution: Res<Resolution>) {
    set_camera(&resolution.0.camera);
    clear_background(BLACK);
}

pub fn post_system(resolution: Res<Resolution>) {
    set_default_camera();
    clear_background(DARKGRAY);
    resolution.0.draw();
}
