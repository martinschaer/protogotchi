pub mod resources;
mod systems;

use bevy::prelude::*;

use systems::{navigation, update};

use resources::SelectState;

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        let state = SelectState {
            display: false,
            entered: f32::MAX,
            debounce: 0.,
            selected: 0,
            options: vec![],
        };
        app.insert_resource(state)
            // TODO: make sure `update` runs after other content has been rendered
            //   label this as "modal", and label other `update`s with "content" and "splash"  
            .add_systems(Update, (navigation, update));
    }
}
