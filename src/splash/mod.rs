mod systems;

use bevy::prelude::*;

use crate::AppState;
use systems::update;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update.run_if(in_state(AppState::Splash)));
    }
}
