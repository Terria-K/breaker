use bevy_ecs::{
    schedule::{Schedule, ShouldRun, Stage, StageLabel, SystemStage},
    system::{Commands, Res},
};
use macroquad::{
    audio::Sound,
    prelude::{vec2, Vec2, WHITE},
    text::Font,
    texture::Texture2D,
};
use macroquad_canvas::Canvas2D;
use std::collections::HashMap;

use crate::{
    component::{
        resource::{ColliderResource, FontResource, Score, Time},
        BallBundle, BallType, LivesTag, PlayerBundle, Resolution, TextBundle, TextScoreBundle,
    },
    system::*,
};

pub const WIDTH: f32 = 800f32;
pub const HEIGHT: f32 = 560f32;

pub type GameResult<T> = anyhow::Result<T>;

pub trait World {
    fn start(&mut self) -> GameResult<()>;
    fn update(&mut self, delta: f32) -> GameResult<()>;
}

#[derive(StageLabel)]
enum Stages {
    Init,
    PreUpdate,
    Update,
    PostUpdate,
    PreDraw,
    Draw,
    PostDraw,
}

#[derive(PartialEq, Eq)]
pub enum States {
    Playing,
    GameOver,
    Restarting,
    Reviving,
    Paused,
}

pub struct WorldScene {
    pub world: bevy_ecs::world::World,
    pub schedule: Schedule,
    pub font: Option<Font>,
    pub score: i32,
}

impl WorldScene {
    pub fn new(
        font: Option<Font>,
        resolution: Canvas2D,
        textures: HashMap<String, Texture2D>,
        audios: HashMap<String, Sound>,
    ) -> Self {
        let schedule = Schedule::default();
        let mut world = bevy_ecs::world::World::new();
        world.insert_resource::<HashMap<String, Texture2D>>(textures);
        world.insert_resource::<HashMap<String, Sound>>(audios);
        world.insert_resource::<Score>(Score(0));
        world.insert_resource::<Time>(Time(0.0));
        world.insert_resource::<States>(States::Reviving);
        world.insert_resource::<Resolution>(Resolution(resolution));
        world.insert_resource::<ColliderResource>(ColliderResource(Vec::new()));
        world.insert_resource::<FontResource>(FontResource(font.unwrap()));
        WorldScene {
            world,
            schedule,
            font,
            score: 0,
        }
    }
}

fn restart(
    mut command: Commands,
    textures: Res<HashMap<String, Texture2D>>,
    states: Res<States>,
) -> ShouldRun {
    match *states {
        States::Restarting => ShouldRun::Yes,
        States::Reviving => {
            command.spawn().insert_bundle(BallBundle::new(
                Vec2::new(180.0, 480.0),
                WHITE,
                textures["Ball"],
                BallType::Original,
            ));
            ShouldRun::Yes
        }
        _ => ShouldRun::No,
    }
}

fn is_playing(states: Res<States>) -> ShouldRun {
    if *states == States::Playing {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

impl World for WorldScene {
    fn start(&mut self) -> GameResult<()> {
        self.schedule
            .add_stage(
                Stages::Init,
                Schedule::default()
                    .with_run_criteria(restart)
                    .with_stage(Stages::Init, SystemStage::parallel()),
            )
            .add_stage(
                Stages::PreUpdate,
                Schedule::default()
                    .with_run_criteria(is_playing)
                    .with_stage(Stages::PreUpdate, SystemStage::parallel())
                    .with_stage(Stages::Update, SystemStage::parallel())
                    .with_stage(Stages::PostUpdate, SystemStage::parallel()),
            )
            .add_stage(Stages::Update, SystemStage::parallel())
            .add_stage(Stages::PreDraw, SystemStage::single_threaded())
            .add_stage(Stages::Draw, SystemStage::single_threaded())
            .add_stage(Stages::PostDraw, SystemStage::single_threaded());

        self.schedule
            .stage(Stages::Init, |schedule: &mut Schedule| {
                schedule.add_system_to_stage(Stages::Init, block::init_system)
            })
            .stage(Stages::PreUpdate, |schedule: &mut Schedule| {
                schedule.add_system_to_stage(Stages::PreUpdate, colliders::query_all_colliders);
                schedule.add_system_to_stage(Stages::Update, ball::system);
                schedule.add_system_to_stage(Stages::Update, player::system);
                schedule.add_system_to_stage(Stages::Update, block::system);
                schedule.add_system_to_stage(Stages::PostUpdate, colliders::system)
            })
            .add_system_to_stage(Stages::Update, text::lives_system)
            .add_system_to_stage(Stages::Update, text::score_system)
            .add_system_to_stage(Stages::Update, text::gameover_system)
            .add_system_to_stage(Stages::Update, pause::system)
            .add_system_to_stage(Stages::PreDraw, draw::pre_system)
            .add_system_to_stage(Stages::Draw, text::system)
            .add_system_to_stage(Stages::Draw, draw::system)
            .add_system_to_stage(Stages::PostDraw, draw::post_system);

        let textures = {
            self.world
                .get_resource::<HashMap<String, Texture2D>>()
                .unwrap()
                .to_owned()
        };

        self.world.spawn().insert_bundle(TextScoreBundle::new(
            "Score".into(),
            false,
            vec2(25.0, 40.0),
            WHITE,
            30u16,
            self.font.unwrap(),
        ));

        self.world
            .spawn()
            .insert_bundle(TextBundle::new(
                "Lives".into(),
                false,
                vec2(WIDTH / 1.2, 40.0),
                WHITE,
                30u16,
                self.font.unwrap(),
            ))
            .insert(LivesTag);
        self.world.spawn().insert_bundle(PlayerBundle::new(
            Vec2::new(120.0, 500.0),
            WHITE,
            textures["Player"],
        ));
        Ok(())
    }

    fn update(&mut self, delta: f32) -> GameResult<()> {
        let mut time = self.world.get_resource_mut::<Time>().unwrap();
        time.0 = delta;
        self.schedule.run(&mut self.world);
        Ok(())
    }
}
