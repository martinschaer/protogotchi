pub mod hardware;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::Rectangle,
    text::Text,
};
use embedded_graphics_framebuf::FrameBuf;
use local_ip_address::local_ip;
// use std::thread::sleep;
use std::time::Duration;

use hardware::resources::Hardware;
use hardware::HardwarePlugin;

pub const W_SIZE: usize = 320;
pub const H_SIZE: usize = 240;

// bg
pub const COLOR_BLUE: Rgb565 = Rgb565::new(9, 14, 21);
// text
const COLOR_LIGHT_BLUE: Rgb565 = Rgb565::new(16, 30, 27);
const COLOR_PURPLE: Rgb565 = Rgb565::new(18, 20, 22);

#[derive(Resource, Default)]
struct GameState {
    text: String,
}

#[derive(Resource)]
struct UIConfig {
    character_style: MonoTextStyle<'static, Rgb565>,
}

fn startup(mut commands: Commands, mut game_state: ResMut<GameState>) {
    commands.insert_resource(UIConfig {
        character_style: MonoTextStyle::new(&FONT_6X10, COLOR_LIGHT_BLUE),
    });
    let char_w = 6_usize;
    let cols = W_SIZE / char_w;
    let line = "**** COMMODORE 64 BASIC V2 ****";
    let line_cols = line.len();
    let line_pad = (cols - line_cols) / 2;
    game_state.text = String::new();
    for _ in 0..line_pad {
        game_state.text.push(' ');
    }
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

fn render_loop(
    time: Res<Time>,
    mut hardware: ResMut<Hardware>,
    ui_config: ResMut<UIConfig>,
    game_state: Res<GameState>,
) {
    // FPS
    // let mut fps = 0_u8;
    // let mut last_time = std::time::Instant::now();
    // let mut now: Instant;
    // let tick = std::time::Duration::from_millis(250);
    // let mut count = 0_u8;
    // fps += 1;
    // now = std::time::Instant::now();
    let elapsed = time.elapsed_seconds_f64();

    // led
    let y = (elapsed.sin() + 1.) * 0.5;
    let stepped_y = (y * 100.).round();
    hardware
        .led_r
        .set_pwm_frequency(50., stepped_y / 100.)
        .unwrap();

    // Backend for the buffer
    let button_a_is_pressed = hardware.button_a.is_low();
    let mut data = [if button_a_is_pressed {
        COLOR_PURPLE
    } else {
        COLOR_BLUE
    }; W_SIZE * H_SIZE];
    let mut fbuf = FrameBuf::new(&mut data, W_SIZE, H_SIZE);

    // Commodore 64 boot screen
    let print_text: String;
    if elapsed % 0.5 < 0.25 {
        print_text = format!("{}█", &game_state.text);
    } else {
        print_text = game_state.text.to_string();
    }
    Text::new(&print_text, Point::new(6, 10), ui_config.character_style)
        .draw(&mut fbuf)
        .unwrap();

    // Write it all to the display
    let area = Rectangle::new(Point::new(0, 0), fbuf.size());
    hardware
        .display
        .lock()
        .unwrap()
        .fill_contiguous(&area, data)
        .unwrap();
}

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        .add_plugins(HardwarePlugin)
        .init_resource::<GameState>()
        .add_systems(Startup, startup)
        .add_systems(Update, render_loop)
        .run();
}
