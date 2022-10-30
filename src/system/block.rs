use std::collections::HashMap;

use bevy_ecs::{
    prelude::Entity,
    system::{Commands, Query, Res, ResMut},
};
use macroquad::{
    prelude::{vec2, Rect, BLACK, BLUE, GREEN, ORANGE, PINK, RED},
    texture::Texture2D,
};

use crate::{
    component::{
        resource::Score, trigger::AddBall, Aabb, BallBundle, BallType, BlockType, Bounty,
        ColorComponent, Lives, Position, Tags, TextureComponent,
    },
    game::{States, WIDTH},
    utils::random,
};

pub fn init_system(
    mut command: Commands,
    textures: Res<HashMap<String, Texture2D>>,
    mut state: ResMut<States>,
) {
    let (width, height) = (6, 6);
    let padding = 0f32;
    let total_block_size = vec2(100.0, 40.0) + vec2(padding, padding);
    let board_start_pos = vec2(
        (WIDTH - (total_block_size.x * width as f32)) * 0.5f32,
        50f32,
    );
    let texture = textures
        .get("Blocks")
        .expect("No textures found!")
        .to_owned();
    let length = width * height;
    command.spawn_batch((0..length).map(move |i| {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        let position = board_start_pos + vec2(block_x, block_y);
        let random = random(0, 100);
        let (block_type, bounty, lives, color) = if random < 80 {
            (BlockType::Fragile, 50, 3, RED)
        } else if random < 90 {
            (BlockType::Cloner, 30, 2, BLUE)
        } else if random < 95 {
            (BlockType::Empty, 0, 0, BLACK)
        } else {
            (BlockType::Special, 100, 1, PINK)
        };

        (
            Position(position),
            Aabb(Rect::new(0.0, 0.0, 100.0, 40.0)),
            ColorComponent(color),
            Tags::Block(block_type),
            Bounty(bounty),
            Lives(lives),
            TextureComponent {
                texture,
                source: Rect::new(0.0, 0.0, 100.0, 40.0),
            },
        )
    }));
    *state = States::Playing;
}

pub fn system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Position,
        &Lives,
        &mut ColorComponent,
        &Bounty,
        &Tags,
    )>,
    mut score: ResMut<Score>,
    textures: Res<HashMap<String, Texture2D>>,
) {
    for (e, pos, lives, mut color, bounty, tags) in query.iter_mut() {
        if let Tags::Block(s) = tags {
            match lives.0 {
                3 => {}
                2 => color.0 = BLUE,
                1 => {
                    if let BlockType::Special = s {
                        continue;
                    }
                    color.0 = ORANGE;
                }
                0 | -1 => {
                    match s {
                        BlockType::Cloner => {
                            let textures = textures.get("Ball").unwrap();
                            let pos = vec2(pos.0.x + 75.0, pos.0.y + 20.0);
                            let bundle = BallBundle::new(pos, BLUE, *textures, BallType::Clone);
                            commands.add(AddBall(bundle));
                        }
                        BlockType::Special => {
                            let textures = textures.get("Ball").unwrap();
                            let pos = vec2(pos.0.x + 75.0, pos.0.y + 20.0);
                            let bundle = BallBundle::new(pos, PINK, *textures, BallType::Special);
                            commands.add(AddBall(bundle));
                        }
                        _ => {}
                    }
                    score.0 += bounty.0;
                    commands.entity(e).despawn();
                }
                _ => color.0 = GREEN,
            }
        }
    }
}
