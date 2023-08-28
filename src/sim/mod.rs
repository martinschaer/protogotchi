use bevy::prelude::*;
use bevy_pixels::prelude::*;
use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};

use crate::{Render, H_SIZE, W_SIZE};

pub struct SimPlugin;

fn setup() {}

fn render_loop(mut wrapper_query: Query<&mut PixelsWrapper>, render: ResMut<Render>) {
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();
    let data = render.data;
    let pixels = data
        .iter()
        .flat_map(|c| {
            let rgb888 = Rgb888::from(*c);
            [rgb888.r(), rgb888.g(), rgb888.b(), 0xff]
        })
        .collect::<Vec<_>>();
    frame.copy_from_slice(&pixels);
}

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PixelsPlugin {
            primary_window: Some(PixelsOptions {
                width: W_SIZE as u32,
                height: H_SIZE as u32,
                scale_factor: 1.,
                auto_resize_buffer: false,
                auto_resize_surface: false,
            }),
        })
        .add_systems(Startup, setup)
        .add_systems(Draw, render_loop);
    }
}
