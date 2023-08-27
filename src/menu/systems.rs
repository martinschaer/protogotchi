use bevy::prelude::*;
use local_ip_address::local_ip;

use super::resources::GameState;
// use crate::W_SIZE;

pub fn startup(mut game_state: ResMut<GameState>) {
    // let char_w = 6_usize;
    // let cols = W_SIZE / char_w;
    let line = "**** COMMODORE 64 BASIC V2 ****";
    // let line_cols = line.len();
    // let line_pad = (cols - line_cols) / 2;
    // game_state.text = String::new();
    // for _ in 0..line_pad {
    //     game_state.text.push(' ');
    // }
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
