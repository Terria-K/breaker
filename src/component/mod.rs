use bevy_ecs::prelude::{Bundle, Component};
use macroquad::{
    prelude::{vec2, Color, Rect, Vec2},
    text::Font,
    texture::Texture2D,
};
use macroquad_canvas::Canvas2D;

use crate::utils::random;
pub mod resource;
pub mod trigger;

// Game World
#[derive(Clone, Copy, Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Aabb(pub Rect);

#[derive(Component)]
pub struct TextureComponent {
    pub texture: Texture2D,
    pub source: Rect,
}

#[derive(Component)]
pub struct ColorComponent(pub Color);

#[derive(Component)]
pub struct Lives(pub i32);

#[derive(Component)]
pub struct Bounty(pub i32);

#[derive(Component, Clone, Copy)]
pub enum Tags {
    Block(BlockType),
    Ball(BallType),
}

#[derive(Clone, Copy)]
pub enum BallType {
    Original,
    Clone,
    Special,
}

#[derive(Clone, Copy)]
pub enum BlockType {
    Fragile,
    Cloner,
    Special,
    Empty,
}

pub struct Resolution(pub Canvas2D);
// UI
#[derive(Component)]
pub struct Text {
    pub text: String,
    pub center: bool,
}

#[derive(Component)]
pub struct FontComponent {
    pub font: Font,
    pub size: u16,
}

#[derive(Component)]
pub struct ScoreTag;

#[derive(Component)]
pub struct GameOverTag;

#[derive(Component)]
pub struct LivesTag;

#[derive(Bundle)]
pub struct TextScoreBundle {
    tag: ScoreTag,
    #[bundle]
    text_bundle: TextBundle,
}

impl TextScoreBundle {
    pub const fn new(
        text: String,
        center: bool,
        position: Vec2,
        color: Color,
        size: u16,
        font: Font,
    ) -> Self {
        TextScoreBundle {
            tag: ScoreTag,
            text_bundle: TextBundle::new(text, center, position, color, size, font),
        }
    }
}

#[derive(Bundle)]
pub struct TextBundle {
    text: Text,
    rect: Position,
    color: ColorComponent,
    font: FontComponent,
}

impl TextBundle {
    pub const fn new(
        text: String,
        center: bool,
        position: Vec2,
        color: Color,
        size: u16,
        font: Font,
    ) -> Self {
        TextBundle {
            text: Text { text, center },
            rect: Position(Vec2 {
                x: position.x,
                y: position.y,
            }),
            color: ColorComponent(color),
            font: FontComponent { font, size },
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    position: Position,
    color: ColorComponent,
    lives: Lives,
    aabb: Aabb,
    texture: TextureComponent,
}

impl PlayerBundle {
    pub const fn new(position: Vec2, color: Color, texture: Texture2D) -> Self {
        PlayerBundle {
            position: Position(position),
            color: ColorComponent(color),
            lives: Lives(3),
            aabb: Aabb(Rect {
                x: 0.0,
                y: 0.0,
                w: 150.0,
                h: 8.0,
            }),
            texture: TextureComponent {
                texture,
                source: Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 150.0,
                    h: 4.0,
                },
            },
        }
    }
}

#[derive(Bundle)]
pub struct BallBundle {
    position: Position,
    color: ColorComponent,
    aabb: Aabb,
    velocity: Velocity,
    tags: Tags,
    texture: TextureComponent,
}

impl BallBundle {
    pub fn new(position: Vec2, color: Color, texture: Texture2D, ball_type: BallType) -> Self {
        let random_vel = vec2(random::<f32>(-1.0, 1.0).signum(), -1.0);
        BallBundle {
            position: Position(position),
            color: ColorComponent(color),
            aabb: Aabb(Rect {
                x: 0.0,
                y: 0.0,
                w: 30.0,
                h: 30.0,
            }),
            texture: TextureComponent {
                texture,
                source: Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 30.0,
                    h: 30.0,
                },
            },
            velocity: Velocity(random_vel),
            tags: Tags::Ball(ball_type),
        }
    }
}
