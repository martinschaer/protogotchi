mod systems;

use bevy::prelude::*;

use crate::AppState;
use systems::{on_enter, render, update};

#[derive(Resource)]
pub struct InputState {
    pub entered: f32,
    pub debounce: f32,
    pub layer: usize,
    pub pointer: usize,
    pub result: String,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            entered: f32::MAX,
            debounce: 0.,
            layer: 0,
            pointer: 0,
            result: String::new(),
        }
    }
}

const LAYER_1: &str = "abcdefghijklmnopqrstuvwxyz";
const LAYER_2: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LAYER_3: &str = "1234567890-=,./;[]\\`";
const LAYER_4: &str = "!@#$%^&*()_+<>?:{}|~";

enum Special {
    Enter(String),
    Space(String),
    Backspace(String),
    Shift(String),
    Symbols(String),
}

#[derive(Resource)]
pub struct Keyboard {
    layer: Vec<Vec<char>>,
    special: Vec<Special>,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        let keyboard = Keyboard {
            layer: vec![
                LAYER_1.chars().collect(),
                LAYER_2.chars().collect(),
                LAYER_3.chars().collect(),
                LAYER_4.chars().collect(),
            ],
            special: vec![
                Special::Enter(String::from("Enter")),
                Special::Space(String::from("Space")),
                Special::Backspace(String::from("<-")),
                Special::Shift(String::from("Shift")),
                Special::Symbols(String::from("!@#")),
            ],
        };
        app.init_resource::<InputState>()
            .insert_resource(keyboard)
            .add_systems(OnEnter(AppState::Input), on_enter)
            .add_systems(Update, (update, render).run_if(in_state(AppState::Input)));
    }
}
