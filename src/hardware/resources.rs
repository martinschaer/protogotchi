use bevy::prelude::*;
use display_interface_spi::SPIInterfaceNoCS;
use mipidsi::Display;
use mipidsi::{models::ST7789, Builder};
use rppal::gpio::{Gpio, InputPin, OutputPin};
use rppal::hal::Delay;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::sync::Mutex;

use crate::{H_SIZE, W_SIZE};

const SPI_DC: u8 = 9;
const BACKLIGHT: u8 = 13;

const BUTTON_A: u8 = 5;
const BUTTON_B: u8 = 6;
const BUTTON_X: u8 = 16;
const BUTTON_Y: u8 = 24;

const LED_R: u8 = 17;
const LED_G: u8 = 27;
const LED_B: u8 = 22;

#[derive(Resource)]
#[allow(dead_code)]
pub struct Hardware {
    pub button_a: InputPin,
    pub button_b: InputPin,
    pub button_x: InputPin,
    pub button_y: InputPin,
    pub led_r: OutputPin,
    pub led_g: OutputPin,
    pub led_b: OutputPin,
    pub backlight: OutputPin,
    pub display: Mutex<Display<SPIInterfaceNoCS<Spi, OutputPin>, ST7789, OutputPin>>,
}

impl Default for Hardware {
    fn default() -> Self {
        let gpio = Gpio::new().unwrap();
        let dc = gpio.get(SPI_DC).unwrap().into_output();
        let mut delay = Delay::new();
        let clock_speed = 60_000_000_u32;
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss1, clock_speed, Mode::Mode0).unwrap();
        let di = SPIInterfaceNoCS::new(spi, dc);
        let display_mutex = Mutex::new(
            Builder::st7789(di)
                .with_display_size(H_SIZE as u16, W_SIZE as u16)
                .with_orientation(mipidsi::Orientation::LandscapeInverted(true))
                .with_invert_colors(mipidsi::ColorInversion::Inverted)
                .init(&mut delay, None::<OutputPin>)
                .unwrap(),
        );

        Hardware {
            button_a: gpio.get(BUTTON_A).unwrap().into_input_pullup(),
            button_b: gpio.get(BUTTON_B).unwrap().into_input_pullup(),
            button_x: gpio.get(BUTTON_X).unwrap().into_input_pullup(),
            button_y: gpio.get(BUTTON_Y).unwrap().into_input_pullup(),
            led_r: gpio.get(LED_R).unwrap().into_output(),
            led_g: gpio.get(LED_G).unwrap().into_output(),
            led_b: gpio.get(LED_B).unwrap().into_output(),
            backlight: gpio.get(BACKLIGHT).unwrap().into_output(),
            display: display_mutex,
        }
    }
}
