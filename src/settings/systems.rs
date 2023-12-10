use bevy::prelude::*;

use crate::plugins::select::resources::{SelectState, StateRoute};

pub fn on_enter(time: Res<Time>, mut state: ResMut<SelectState>) {
    state.display = true;
    state.entered = time.elapsed_seconds();
    state.selected = 0;
    state.options = vec![
        StateRoute {
            label: String::from("Wi-Fi"),
            route: String::from("wifi"),
        },
        StateRoute {
            label: String::from("Back"),
            route: String::from("main"),
        },
    ];
}
