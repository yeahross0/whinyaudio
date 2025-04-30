#![cfg(all(target_os = "unknown", target_arch = "wasm32"))]

use macroquad::logging as log;
use macroquad::prelude::*;
use tinyaudio::prelude::*;

#[macroquad::main("whinyaudio example")]
async fn main() {
    log::debug!("whinyaudio example");

    while !is_mouse_button_pressed(MouseButton::Left) {
        clear_background(ORANGE);
        next_frame().await;
    }

    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 48000,
        channel_sample_count: 48000 / 4,
    };

    let _device = run_output_device(params, {
        move |data| {
            for samples in data.chunks_mut(params.channels_count) {
                for sample in samples {
                    *sample = rand::gen_range(-1.0, 1.0);
                }
            }
        }
    })
    .unwrap();

    loop {
        clear_background(RED);

        next_frame().await;
    }
}
