use bevy::prelude::*;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::*,
    text::Text,
};
use embedded_graphics_framebuf::FrameBuf;
use local_ip_address::local_ip;

use super::resources::{MenuState, UIConfig};
use crate::{AppState, CurrentRouteState, Render, StateRoute, COLOR_BG, COLOR_FG, H_SIZE, W_SIZE};

pub fn startup(mut commands: Commands, mut game_state: ResMut<MenuState>) {
    commands.insert_resource(UIConfig {
        character_style: MonoTextStyle::new(&FONT_6X10, COLOR_FG),
    });

    let line = "**** Welcome to the Protogotchi terminal ^_^ ****\n\n";
    game_state.text.push_str(line);

    // get IP
    let hostname = std::process::Command::new("hostname").output().unwrap();
    let hostname = hostname.stdout;
    let hostname = String::from_utf8(hostname).unwrap();
    let my_local_ip = match local_ip() {
        Ok(ip) => ip.to_string(),
        Err(x) => format!("{:?}", x),
    };
    game_state.text.push_str(&format!(
        "hostname: {}\nIP: {}\n\nPress X for Settings\n\n",
        hostname, my_local_ip
    ));
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

    let print_text = if elapsed % 0.5 < 0.25 {
        format!("{}_", &game_state.text)
    } else {
        game_state.text.to_string()
    };
    render.data.fill(COLOR_BG);
    let mut fbuf = FrameBuf::new(&mut render.data, W_SIZE, H_SIZE);
    Text::new(&print_text, Point::new(6, 10), ui_config.character_style)
        .draw(&mut fbuf)
        .unwrap();
}

pub fn navigation(
    time: Res<Time>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut route_state: ResMut<CurrentRouteState>,
    render: Res<Render>,
    state: Res<MenuState>,
) {
    let now = time.elapsed_seconds();
    if now > 0.2 + state.entered && render.button_x_pressed {
        app_state_next_state.set(AppState::Settings);
        route_state.route = StateRoute {
            label: String::from("Settings"),
            route: String::from("settings"),
        };
        route_state.params = vec![];
    }
}
