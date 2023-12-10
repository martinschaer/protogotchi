use bevy::prelude::*;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::*,
    primitives::{rectangle::Rectangle, PrimitiveStyleBuilder},
    text::Text,
};
use embedded_graphics_framebuf::FrameBuf;

use crate::{
    plugins::select::parse_route, AppState, CurrentRouteState, Render, COLOR_BG, COLOR_FG, DB,
    H_SIZE, W_SIZE,
};

use super::{InputState, Keyboard, Special};

pub fn on_enter(time: Res<Time>, mut state: ResMut<InputState>) {
    state.entered = time.elapsed_seconds();
    println!("input entered");
}

fn build_button(
    label: &str,
    i: i32,
    cols: f32,
    col_w: f32,
    row_h: f32,
) -> embedded_graphics::text::Text<
    '_,
    embedded_graphics::mono_font::MonoTextStyle<'_, embedded_graphics::pixelcolor::Rgb565>,
> {
    Text::new(
        label,
        Point::new(
            (((i as f32 % cols) + 0.5) * col_w) as i32 - (label.len() as f32 * 0.5 * 6.) as i32,
            (((i as f32 / cols).floor() + 0.5) * row_h) as i32,
        ),
        MonoTextStyle::new(&FONT_6X10, COLOR_FG),
    )
}

pub fn render(state: Res<InputState>, keyboard: Res<Keyboard>, mut render: ResMut<Render>) {
    render.data.fill(COLOR_BG);
    let mut fbuf = FrameBuf::new(&mut render.data, W_SIZE, H_SIZE);
    let keys = &keyboard.layer[state.layer];
    let total_keys = (keys.len() + keyboard.special.len()) as f32;
    let cols = total_keys.sqrt().ceil();
    let rows = (total_keys / cols).ceil() + 1.;
    let col_w = (W_SIZE as f32) / cols;
    let row_h = (H_SIZE as f32) / rows;

    // layer buttons
    for (i, key) in keys.iter().enumerate() {
        Text::new(
            key.to_string().as_ref(),
            Point::new(
                (((i as f32 % cols) + 0.5) * col_w) as i32 - 3,
                (((i as f32 / cols).floor() + 0.5) * row_h) as i32,
            ),
            MonoTextStyle::new(&FONT_6X10, COLOR_FG),
        )
        .draw(&mut fbuf)
        .unwrap();
    }

    // special
    for (i, special) in keyboard.special.iter().enumerate() {
        match special {
            Special::Enter(s)
            | Special::Shift(s)
            | Special::Symbols(s)
            | Special::Backspace(s)
            | Special::Space(s) => {
                build_button(s, (i + keys.len()) as i32, cols, col_w, row_h)
                    .draw(&mut fbuf)
                    .unwrap();
            }
        }
    }

    // indicator
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(COLOR_FG)
        .stroke_width(2)
        .build();
    Rectangle::new(
        Point::new(
            (((state.pointer as f32 % cols) + 0.5) * col_w) as i32 - 5,
            (((state.pointer as f32 / cols).floor() + 0.5) * row_h) as i32 - 7,
        ),
        Size::new(10, 14),
    )
    .into_styled(style)
    .draw(&mut fbuf)
    .unwrap();

    // result
    Text::new(
        &state.result,
        Point::new((col_w * 0.5) as i32 - 3, ((rows - 0.5) * row_h) as i32 - 5),
        MonoTextStyle::new(&FONT_6X10, COLOR_FG),
    )
    .draw(&mut fbuf)
    .unwrap();
}

pub fn update(
    time: Res<Time>,
    render: Res<Render>,
    keyboard: Res<Keyboard>,
    mut db: ResMut<DB>,
    mut input_state: ResMut<InputState>,
    mut route_state: ResMut<CurrentRouteState>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    let now = time.elapsed_seconds();
    if now > 0.2 + input_state.entered && now > input_state.debounce + 0.2 {
        if render.button_a_pressed {
            if input_state.pointer == 0 {
                input_state.pointer =
                    keyboard.layer[input_state.layer].len() + keyboard.special.len() - 1;
            } else {
                input_state.pointer -= 1;
            }
            input_state.debounce = now;
        } else if render.button_b_pressed {
            input_state.pointer += 1;
            if input_state.pointer
                >= keyboard.layer[input_state.layer].len() + keyboard.special.len()
            {
                input_state.pointer = 0;
            }
            input_state.debounce = now;
        } else if render.button_x_pressed {
            let layer = &keyboard.layer[input_state.layer];
            if input_state.pointer < layer.len() {
                let char = layer[input_state.pointer];
                input_state.result.push(char);
            } else {
                let special = &keyboard.special[input_state.pointer - layer.len()];
                match special {
                    Special::Enter(_) => {
                        if !route_state.params.is_empty() {
                            db.records
                                .insert(route_state.params[0].clone(), input_state.result.clone());
                            input_state.result.clear();
                        }
                        // TODO: use a router fn
                        let (goto_app_state, params) = if route_state.params.len() > 1 {
                            parse_route(&route_state.params[1])
                        } else {
                            (AppState::Settings, vec![])
                        };
                        app_state_next_state.set(goto_app_state);
                        route_state.params = params;
                        // --
                    }
                    Special::Backspace(_) => {
                        input_state.result.pop();
                    }
                    Special::Space(_) => {
                        input_state.result.push(' ');
                    }
                    Special::Shift(_) => {
                        input_state.layer = match input_state.layer {
                            0 => 1,
                            1 => 0,
                            2 => 3,
                            3 => 2,
                            other => other,
                        };
                        let total_keys = (keyboard.layer[input_state.layer].len()
                            + keyboard.special.len())
                            as f32;
                        input_state.pointer = ((total_keys - 1.) as usize).min(input_state.pointer);
                    }
                    Special::Symbols(_) => {
                        input_state.layer = match input_state.layer {
                            0 => 2,
                            1 => 3,
                            2 => 0,
                            3 => 1,
                            other => other,
                        };
                        let total_keys = (keyboard.layer[input_state.layer].len()
                            + keyboard.special.len())
                            as f32;
                        input_state.pointer = ((total_keys - 1.) as usize).min(input_state.pointer);
                    }
                }
            }
            input_state.debounce = now;
        } else if render.button_y_pressed {
            // TODO: use a router fn
            app_state_next_state.set(AppState::Settings);
            route_state.params = vec![];
            // --
        }
    }
}
