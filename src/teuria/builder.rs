use std::collections::HashMap;

use macroquad::{
    audio::{load_sound, Sound},
    prelude::*,
};
use macroquad_canvas::Canvas2D;

use crate::game::WorldScene;

pub struct GameWorldBuilder {
    textures: HashMap<String, Texture2D>,
    audios: HashMap<String, Sound>,
    font: Option<Font>,
    resolution: Canvas2D,
}

impl GameWorldBuilder {
    pub fn new(resolution: Canvas2D) -> GameWorldBuilder {
        GameWorldBuilder {
            textures: HashMap::new(),
            audios: HashMap::new(),
            font: None,
            resolution,
        }
    }

    pub async fn add_audio(
        &mut self,
        audio_name: String,
        audio_path: &str,
    ) -> Result<&mut GameWorldBuilder, FileError> {
        self.audios
            .insert(audio_name, load_sound(audio_path).await?);
        Ok(self)
    }

    pub async fn add_texture(
        &mut self,
        texture_name: String,
        texture_path: &str,
    ) -> Result<&mut GameWorldBuilder, FileError> {
        self.textures
            .insert(texture_name, load_texture(texture_path).await?);
        Ok(self)
    }

    pub fn insert_font(&mut self, font: Font) -> &mut GameWorldBuilder {
        self.font = Some(font);
        self
    }

    pub fn build(&mut self) -> WorldScene {
        let textures = self.textures.clone();
        WorldScene::new(self.font, self.resolution, textures, self.audios.clone())
    }
}
