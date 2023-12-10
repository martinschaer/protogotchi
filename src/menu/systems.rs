use bevy::prelude::*;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::*,
    text::Text,
};
use embedded_graphics_framebuf::FrameBuf;
use local_ip_address::local_ip;
use std::io::Result;

use super::resources::{MenuState, UIConfig};
use crate::{AppState, CurrentRouteState, Render, COLOR_BG, COLOR_FG, DB, H_SIZE, W_SIZE};

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

#[cfg(target_os = "linux")]
fn wifi_connect(ssid: &str, pass: &str) -> Result<String> {
    // read wpa_supplicant.conf
    let mut content = std::fs::read_to_string("/etc/wpa_supplicant/wpa_supplicant.conf")?;
    let psk = std::process::Command::new("wpa_passphrase")
        .args([ssid, pass])
        .output()
        .unwrap();
    let psk = String::from_utf8(psk.stdout).unwrap();

    content.push_str(&psk);
    std::fs::write(format!("/etc/wpa_supplicant/{}.conf", ssid), content).unwrap();
    // backup wpa_supplicant.conf
    _ = std::fs::copy(
        "/etc/wpa_supplicant/wpa_supplicant.conf",
        "/etc/wpa_supplicant/wpa_supplicant.conf.bak",
    );

    // update wpa_supplicant.conf
    _ = std::fs::copy(
        format!("/etc/wpa_supplicant/{}.conf", ssid),
        "/etc/wpa_supplicant/wpa_supplicant.conf",
    );

    // find network id
    let out = std::process::Command::new("wpa_cli")
        .args(["-i", "wlan0", "list_networks", "|", "grep", ssid])
        .output()?;
    let network_id = String::from_utf8(out.stdout).unwrap();
    let network_id = network_id.split_whitespace().collect::<Vec<_>>()[0];

    // select network
    let out = std::process::Command::new("wpa_cli")
        .args(["-i", "wlan0", "select_network", network_id])
        .output()?;
    Ok(String::from_utf8(out.stdout).unwrap())
}

pub fn on_enter(
    time: Res<Time>,
    route_state: ResMut<CurrentRouteState>,
    db: Res<DB>,
    mut state: ResMut<MenuState>,
) {
    state.entered = time.elapsed_seconds();
    println!("menu entered");

    if !route_state.params.is_empty() && route_state.params[0] == "connect" {
        // sudo nmcli --ask dev wifi connect <example_ssid>
        let ssid = match db.records.get("wifi.ssid") {
            Some(s) => s,
            None => "",
        };
        let pass = match db.records.get("wifi.password") {
            Some(p) => p,
            None => "",
        };

        #[cfg(target_os = "linux")]
        let out = wifi_connect(ssid, pass);

        #[cfg(target_os = "macos")]
        let out = std::process::Command::new("networksetup")
            .args(["-setairportnetwork", "en0", ssid, pass])
            .spawn();

        #[cfg(target_os = "windows")]
        let out = std::process::Command::new("netsh")
            .args(["wlan", "connect", ssid, "password", pass])
            .spawn();

        match out {
            Ok(_) => {
                state
                    .text
                    .push_str(&format!("Connecting to network {}...\n", &ssid));
            }
            Err(x) => {
                state.text.push_str(&format!("Error: {}\n", x));
            }
        }
    }
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
        // TODO: use a router fn
        app_state_next_state.set(AppState::Settings);
        route_state.params = vec![];
        // --
    }
}
