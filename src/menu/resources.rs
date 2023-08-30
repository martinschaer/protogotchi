use bevy::prelude::*;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;

#[derive(Resource)]
pub struct UIConfig {
    pub character_style: MonoTextStyle<'static, Rgb565>,
}

#[derive(Resource)]
pub struct MenuState {
    pub text: String,
    pub entered: f32,
}

impl Default for MenuState {
    fn default() -> Self {
        MenuState {
            text: String::new(),
            entered: f32::MAX,
        }
    }
}
