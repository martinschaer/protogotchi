mod systems;

use bevy::prelude::*;

use crate::AppState;
use systems::{on_enter, update};

pub struct SplashPlugin;

#[derive(Resource)]
pub struct SplashState {
    pub entered: f32,
}

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SplashState { entered: f32::MAX })
            .add_systems(OnEnter(AppState::Splash), on_enter)
            .add_systems(Update, update.run_if(in_state(AppState::Splash)));
    }
}
