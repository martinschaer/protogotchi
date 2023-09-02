use bevy::prelude::*;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::*,
    text::Text,
};
use embedded_graphics_framebuf::FrameBuf;
use local_ip_address::local_ip;

use super::resources::{MenuState, UIConfig};
use crate::{AppState, Render, COLOR_BG, COLOR_FG, H_SIZE, W_SIZE};

pub fn startup(mut commands: Commands, mut game_state: ResMut<MenuState>) {
    commands.insert_resource(UIConfig {
        character_style: MonoTextStyle::new(&FONT_6X10, COLOR_FG),
    });

    let line = "**** COMMODORE 64 BASIC V2 ****";
    game_state.text.push_str(line);
    game_state
        .text
        .push_str("\n\n 64K RAM SYSTEM  38911 BASIC BYTES FREE\n\nREADY.\n");

    // get IP
    let hostname = std::process::Command::new("hostname").output().unwrap();
    let hostname = hostname.stdout;
    let hostname = String::from_utf8(hostname).unwrap();
    let my_local_ip = local_ip().unwrap();
    game_state
        .text
        .push_str(&format!("hostname: {}\nIP: {}\n", hostname, my_local_ip));
}

pub fn on_enter(time: Res<Time>, mut state: ResMut<MenuState>) {
    state.entered = time.elapsed_seconds();
    println!("menu entered");
}

pub fn render_loop(
    time: Res<Time>,
    ui_config: ResMut<UIConfig>,
    game_state: Res<MenuState>,
    mut render: ResMut<Render>,
) {
    let elapsed = time.elapsed_seconds_f64();

    let print_text: String;
    if elapsed % 0.5 < 0.25 {
        print_text = format!("{}█", &game_state.text);
    } else {
        print_text = game_state.text.to_string();
    }
    render.data.fill(COLOR_BG);
    let mut fbuf = FrameBuf::new(&mut render.data, W_SIZE, H_SIZE);
    Text::new(&print_text, Point::new(6, 10), ui_config.character_style)
        .draw(&mut fbuf)
        .unwrap();
}

pub fn navigation(
    time: Res<Time>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    render: Res<Render>,
    state: Res<MenuState>,
) {
    let now = time.elapsed_seconds();
    if now > 0.2 + state.entered {
        if render.button_x_pressed {
            app_state_next_state.set(AppState::Settings);
        }
    }
}
