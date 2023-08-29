use bevy::prelude::*;

use crate::Render;
use crate::COLOR_PURPLE;

pub fn update(mut render: ResMut<Render>) {
    render.data.fill(COLOR_PURPLE)
}
