use bevy::prelude::*;

use crate::plugins::select::resources::{SelectState, StateRoute};

pub fn on_enter(time: Res<Time>, mut state: ResMut<SelectState>) {
    state.display = true;
    state.entered = time.elapsed_seconds();
    state.selected = 0;
    state.options = vec![
        StateRoute {
            label: String::from("Network Name"),
            route: String::from("input: wifi.ssid"),
        },
        StateRoute {
            label: String::from("Password"),
            route: String::from("input: wifi.password"),
        },
        StateRoute {
            label: String::from("Back"),
            route: String::from("settings"),
        },
    ];
}
