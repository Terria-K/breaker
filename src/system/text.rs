use bevy_ecs::{
    prelude::Entity,
    query::{With, Without},
    system::{Commands, Query, Res, ResMut},
};
use macroquad::{
    prelude::{is_key_pressed, KeyCode},
    text::{draw_text_ex, measure_text, TextParams},
};

use crate::{
    component::{
        resource::Score, trigger::AddPlayerLives, ColorComponent, FontComponent, GameOverTag,
        Lives, LivesTag, Position, ScoreTag, Tags, Text,
    },
    game::States,
};

type TextQuery<'a> = (
    &'a Text,
    &'a ColorComponent,
    &'a FontComponent,
    &'a Position,
);

type MutableTextQuery<'a> = &'a mut Text;

pub fn system(query: Query<TextQuery>) {
    for (text, color, font, rect) in query.iter() {
        let offset = if text.center {
            measure_text(&text.text, Some(font.font), font.size, 1.0).width
        } else {
            0.0
        };
        draw_text_ex(
            &text.text,
            rect.0.x - offset,
            rect.0.y,
            TextParams {
                font: font.font,
                font_size: font.size,
                color: color.0,
                ..Default::default()
            },
        )
    }
}

pub fn gameover_system(
    mut command: Commands,
    mut query: Query<(Entity, MutableTextQuery), With<GameOverTag>>,
    ball_blocks_query: Query<Entity, With<Tags>>,
    mut state: ResMut<States>,
    mut score: ResMut<Score>,
) {
    for (entity, mut text) in query.iter_mut() {
        text.text = "Game Over!".into();
        if is_key_pressed(KeyCode::Space) {
            command.entity(entity).despawn();
            for e in ball_blocks_query.iter() {
                command.entity(e).despawn();
                command.add(AddPlayerLives(3));
            }
            score.0 = 0;
            *state = States::Reviving;
        }
    }
}

pub fn score_system(score: Res<Score>, mut query: Query<MutableTextQuery, With<ScoreTag>>) {
    for mut text in query.iter_mut() {
        let score = score.0;
        let tex = format!("Score: {score}");
        text.text = tex;
    }
}

pub fn lives_system(
    mut player_query: Query<&Lives, Without<Tags>>,
    mut query: Query<MutableTextQuery, With<LivesTag>>,
) {
    let lives = player_query.get_single_mut();
    if let Ok(lives) = lives {
        let lives = lives.0;
        for mut text in query.iter_mut() {
            text.text = format!("Lives: {lives}");
        }
    }
}
