pub mod resources;
mod systems;

use bevy::prelude::*;

use crate::AppState;
use resources::SelectState;
use systems::{navigation, update};

pub fn parse_route(route: &str) -> (AppState, Vec<String>) {
    let mut parts = route.split(':');
    let state_name = parts.next().unwrap().trim();
    let params: Vec<String> = match parts.next() {
        Some(s) => s.split(',').map(|s| s.trim().to_owned()).collect(),
        None => vec![],
    };
    let state = match state_name {
        "menu" => AppState::Menu,
        "settings" => AppState::Settings,
        "wifi" => AppState::Wifi,
        "input" => AppState::Input,
        "connect" => AppState::Menu,
        _ => AppState::Menu,
    };
    (state, params)
}

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        let state = SelectState {
            display: false,
            entered: f32::MAX,
            debounce: 0.,
            selected: 0,
            options: vec![],
        };
        app.insert_resource(state)
            // TODO: make sure `update` runs after other content has been rendered
            //   label this as "modal", and label other `update`s with "content" and "splash"
            .add_systems(Update, (navigation, update));
    }
}
