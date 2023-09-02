use bevy::prelude::*;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::*,
    text::Text,
};
use embedded_graphics_framebuf::FrameBuf;

use crate::AppState;
use crate::Render;
use crate::COLOR_BG;
use crate::COLOR_FG;
use crate::H_SIZE;
use crate::W_SIZE;

use super::resources::{Setting, SettingsState};

pub fn on_enter(time: Res<Time>, mut state: ResMut<SettingsState>) {
    state.entered = time.elapsed_seconds();
    println!("settings entered");
}

pub fn update(mut render: ResMut<Render>, state: Res<SettingsState>) {
    render.data.fill(COLOR_FG);

    let print_text = match state.selected {
        Setting::Wifi => "> Wifi\n  Back",
        Setting::Back => "  Wifi\n> Back",
    };
    render.data.fill(COLOR_BG);
    let mut fbuf = FrameBuf::new(&mut render.data, W_SIZE, H_SIZE);
    Text::new(
        &print_text,
        Point::new(6, 10),
        MonoTextStyle::new(&FONT_6X10, COLOR_FG),
    )
    .draw(&mut fbuf)
    .unwrap();
}

pub fn navigation(
    time: Res<Time>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    render: Res<Render>,
    mut state: ResMut<SettingsState>,
) {
    let now = time.elapsed_seconds();
    if now > 0.2 + state.entered && now > state.debounce + 0.2 {
        if render.button_x_pressed {
            match state.selected {
                Setting::Wifi => {
                    // app_state_next_state.set(AppState::Menu);
                }
                Setting::Back => {
                    app_state_next_state.set(AppState::Menu);
                }
            }
            state.debounce = now;
        } else if render.button_a_pressed {
            match state.selected {
                Setting::Wifi => {
                    state.selected = Setting::Back;
                }
                Setting::Back => {
                    state.selected = Setting::Wifi;
                }
            }
            state.debounce = now;
        } else if render.button_b_pressed {
            match state.selected {
                Setting::Wifi => {
                    state.selected = Setting::Back;
                }
                Setting::Back => {
                    state.selected = Setting::Wifi;
                }
            }
            state.debounce = now;
        }
    }
}
