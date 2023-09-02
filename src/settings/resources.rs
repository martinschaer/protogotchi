use bevy::prelude::*;

pub enum Setting {
    Wifi,
    Back,
}

#[derive(Resource)]
pub struct SettingsState {
    pub entered: f32,
    pub debounce: f32,
    pub selected: Setting,
}

impl Default for SettingsState {
    fn default() -> Self {
        SettingsState {
            entered: f32::MAX,
            debounce: 0.,
            selected: Setting::Wifi,
        }
    }
}
