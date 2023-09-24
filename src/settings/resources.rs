use bevy::prelude::*;

#[derive(Resource)]
pub struct SettingsState {
    pub entered: f32,
    pub debounce: f32,
    pub selected: usize,
    pub options: Vec<String>,
}
