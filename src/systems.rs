use bevy::prelude::*;

use crate::AppState;
use crate::Render;

pub fn transition_to_splash_state(
    render: Res<Render>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if render.button_a_pressed && *app_state.get() != AppState::Splash {
        app_state_next_state.set(AppState::Splash);
    }
}
