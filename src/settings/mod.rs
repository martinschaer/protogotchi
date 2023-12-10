mod systems;
pub mod wifi;

use bevy::prelude::*;

use crate::AppState;
use systems::on_enter;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Settings), on_enter);
    }
}
