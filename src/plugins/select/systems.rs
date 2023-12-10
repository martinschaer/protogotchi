use bevy::prelude::*;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::*,
    text::Text,
};
use embedded_graphics_framebuf::FrameBuf;

use crate::Render;
use crate::COLOR_BG;
use crate::COLOR_FG;
use crate::H_SIZE;
use crate::W_SIZE;
use crate::{AppState, CurrentRouteState};

use super::{parse_route, resources::SelectState};

pub fn update(mut render: ResMut<Render>, state: Res<SelectState>) {
    if state.display {
        // render.data.fill(COLOR_FG);
        let print_text =
            state
                .options
                .iter()
                .enumerate()
                .fold(String::from(""), |mut acc, (i, o)| {
                    if state.selected == i {
                        acc.push_str("\n> ");
                    } else {
                        acc.push_str("\n  ");
                    }
                    acc.push_str(&o.label);
                    acc
                });
        render.data.fill(COLOR_BG);
        let mut fbuf = FrameBuf::new(&mut render.data, W_SIZE, H_SIZE);
        Text::new(
            &print_text,
            Point::new(6, 10),
            MonoTextStyle::new(&FONT_6X10, COLOR_FG),
        )
        .draw(&mut fbuf)
        .unwrap();
    }
}

pub fn navigation(
    time: Res<Time>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    render: Res<Render>,
    mut state: ResMut<SelectState>,
    mut route_state: ResMut<CurrentRouteState>,
) {
    let now = time.elapsed_seconds();
    if now > 0.2 + state.entered && now > state.debounce + 0.2 && state.display {
        if render.button_x_pressed {
            // routing
            let state_route = &state.options[state.selected];
            let (goto_app_state, params) = parse_route(&state_route.route);
            app_state_next_state.set(goto_app_state);
            route_state.params = params;

            state.display = false;
            state.debounce = now;
        } else if render.button_a_pressed {
            if state.selected == 0 {
                state.selected = state.options.len() - 1;
            } else {
                state.selected -= 1;
            }
            state.debounce = now;
        } else if render.button_b_pressed {
            state.selected += 1;
            if state.selected >= state.options.len() {
                state.selected = 0;
            }
            state.debounce = now;
        }
    }
}
