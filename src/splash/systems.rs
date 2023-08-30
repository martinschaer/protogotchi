use bevy::prelude::*;

use crate::AppState;
use crate::Render;
use crate::COLOR_PURPLE;

use super::SplashState;

pub fn on_enter(time: Res<Time>, mut state: ResMut<SplashState>) {
    state.entered = time.elapsed_seconds();
    println!("splash entered");
}

pub fn update(
    time: Res<Time>,
    mut render: ResMut<Render>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    state: Res<SplashState>,
) {
    render.data.fill(COLOR_PURPLE);
    let now = time.elapsed_seconds();
    if now > 5. + state.entered {
        app_state_next_state.set(AppState::Menu);
    }
}
