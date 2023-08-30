use bevy::prelude::*;

use crate::AppState;
use crate::Render;
use crate::COLOR_LIGHT_BLUE;

use super::resources::SettingsState;

pub fn on_enter(time: Res<Time>, mut state: ResMut<SettingsState>) {
    state.entered = time.elapsed_seconds();
    println!("settings entered");
}

pub fn update(mut render: ResMut<Render>) {
    render.data.fill(COLOR_LIGHT_BLUE)
}

pub fn navigation(
    time: Res<Time>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    render: Res<Render>,
    state: Res<SettingsState>,
) {
    let now = time.elapsed_seconds();
    if now > 0.2 + state.entered {
        if render.button_x_pressed {
            app_state_next_state.set(AppState::Menu);
        }
    }
}
