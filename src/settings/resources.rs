use bevy::prelude::*;

#[derive(Resource)]
pub struct SettingsState {
    pub entered: f32,
}

impl Default for SettingsState {
    fn default() -> Self {
        SettingsState {
            entered: f32::MAX,
        }
    }
}
