use bevy::prelude::*;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;

#[derive(Resource)]
pub struct UIConfig {
    pub character_style: MonoTextStyle<'static, Rgb565>,
}

#[derive(Resource, Default)]
pub struct GameState {
    pub text: String,
    pub button_a_pressed: bool,
}
