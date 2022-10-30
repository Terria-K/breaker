use game::World;
use game::{HEIGHT, WIDTH};
use macroquad::{miniquad::conf::Icon, prelude::*};
use macroquad_canvas::Canvas2D;
use teuria::builder::GameWorldBuilder;

mod component;
mod game;
mod system;
mod teuria;
mod utils;

fn config() -> Conf {
    let small = Image::from_file_with_format(include_bytes!("../res/icon16.png"), None);
    let medium = Image::from_file_with_format(include_bytes!("../res/icon32.png"), None);
    let big = Image::from_file_with_format(include_bytes!("../res/icon64.png"), None);

    Conf {
        window_title: "Breaker".into(),
        window_width: 1024,
        window_height: 620,
        icon: Some(Icon {
            small: small.bytes.try_into().unwrap(),
            medium: medium.bytes.try_into().unwrap(),
            big: big.bytes.try_into().unwrap(),
        }),
        high_dpi: true,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() -> anyhow::Result<()> {
    rand::srand(macroquad::miniquad::date::now() as u64);
    let delta = get_frame_time();
    let font = load_ttf_font("res/Rubik-Light.ttf").await?;

    let mut game_world = GameWorldBuilder::new(Canvas2D::new(WIDTH, HEIGHT))
        .insert_font(font)
        .add_audio("HitBlock".into(), "res/hitblock.ogg")
        .await?
        .add_texture("Player".into(), "res/player.png")
        .await?
        .add_texture("Blocks".into(), "res/blocks-Sheet.png")
        .await?
        .add_texture("Ball".into(), "res/ball.png")
        .await?
        .build();
    game_world.start()?;

    loop {
        game_world.update(delta)?;
        next_frame().await;
    }
}
