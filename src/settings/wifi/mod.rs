mod systems;

use bevy::prelude::*;

use crate::AppState;
use systems::on_enter;

pub struct WifiPlugin;

impl Plugin for WifiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Wifi), on_enter);
    }
}
