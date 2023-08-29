use bevy::prelude::*;

use crate::AppState;
use crate::Render;

pub fn transition_to_splash_state(
    render: Res<Render>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if render.button_a_pressed {
        if app_state.get().to_owned() != AppState::Splash {
            app_state_next_state.set(AppState::Splash);
            println!("Entered AppState::Splash");
        }
    }
}

pub fn transition_to_menu_state(
    render: Res<Render>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if !render.button_a_pressed {
        if app_state.get().to_owned() != AppState::Menu {
            app_state_next_state.set(AppState::Menu);
            println!("Entered AppState::Menu");
        }
    }
}
