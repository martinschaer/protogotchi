mod resources;
mod systems;

use bevy::prelude::*;

use crate::AppState;
use resources::SettingsState;
use systems::{navigation, on_enter, update};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SettingsState>()
            .add_systems(OnEnter(AppState::Settings), on_enter)
            .add_systems(
                Update,
                (navigation, update).run_if(in_state(AppState::Settings)),
            );
    }
}
