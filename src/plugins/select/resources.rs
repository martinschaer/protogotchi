use bevy::prelude::*;

#[derive(Resource)]
pub struct SelectState {
    pub display: bool,
    pub entered: f32,
    pub debounce: f32,
    pub selected: usize,
    pub options: Vec<String>,
}
