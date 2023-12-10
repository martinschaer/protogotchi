use bevy::prelude::*;

#[derive(Clone)]
pub struct StateRoute {
    pub label: String,
    pub route: String,
}

#[derive(Resource)]
pub struct SelectState {
    pub display: bool,
    pub entered: f32,
    pub debounce: f32,
    pub selected: usize,
    pub options: Vec<StateRoute>,
}
