use std::collections::HashMap;

use bevy_ecs::{query::Without, system::Command};
use macroquad::{
    prelude::{vec2, Vec2, WHITE},
    text::Font,
    texture::Texture2D,
};

use crate::game::WIDTH;

use super::{BallBundle, GameOverTag, Lives, Position, Tags, TextBundle};

pub struct AddPlayer;
pub struct AddBall(pub BallBundle);
pub struct AddGameOverText(pub Font);
pub struct AddPlayerLives(pub i32);

impl Command for AddPlayer {
    fn write(self, world: &mut bevy_ecs::world::World) {
        let mut query = world.query_filtered::<(&Position, &mut Lives), Without<Tags>>();
        let textures = {
            world
                .get_resource::<HashMap<String, Texture2D>>()
                .unwrap()
                .to_owned()
        };
        let textures = textures.get("Ball").unwrap();
        let result = { query.get_single_mut(world) };
        if let Ok((pos, mut lives)) = result {
            let pos = Vec2::new(pos.0.x + 60.0, pos.0.y - 20.0);
            lives.0 -= 1;
            if lives.0 <= 0 {
                return;
            }
            world.spawn().insert_bundle(BallBundle::new(
                pos,
                WHITE,
                *textures,
                super::BallType::Original,
            ));
        }
    }
}

impl Command for AddBall {
    fn write(self, world: &mut bevy_ecs::world::World) {
        world.spawn().insert_bundle(self.0);
    }
}

impl Command for AddGameOverText {
    fn write(self, world: &mut bevy_ecs::world::World) {
        world
            .spawn()
            .insert_bundle(TextBundle::new(
                "Game Over!".into(),
                true,
                vec2(WIDTH / 1.6, 40.0),
                WHITE,
                30,
                self.0,
            ))
            .insert(GameOverTag);
    }
}

impl Command for AddPlayerLives {
    fn write(self, world: &mut bevy_ecs::world::World) {
        let mut query = world.query_filtered::<&mut Lives, Without<Tags>>();
        let player = query.get_single_mut(world);
        if let Ok(mut player) = player {
            player.0 = self.0;
        }
    }
}
