use bevy::prelude::*;

use crate::plugins::select::resources::SelectState;

pub fn on_enter(time: Res<Time>, mut state: ResMut<SelectState>) {
    state.display = true;
    state.entered = time.elapsed_seconds();
    state.selected = 0;
    state.options = vec![String::from("Wifi"), String::from("Back")];
}
