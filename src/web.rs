//! WebAssembly output device

#![cfg(all(target_os = "unknown", target_arch = "wasm32"))]

use crate::{AudioOutputDevice, BaseAudioOutputDevice, OutputDeviceParameters};
use std::{cell::Cell, error::Error};

thread_local! {
    static CALLBACK: Cell<Option<Box<dyn FnMut(&mut [f32]) + Send + 'static>>> = Cell::new(None);

    static DATA: Cell<Vec<f32>> = Cell::new(Vec::new());
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn init_audio_output(sample_rate: u32, channels_count: u32, channel_sample_count: u32) -> i32;
    fn close_audio_output();
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
unsafe extern "C" fn write_samples() -> *const f32 {
    let mut func = CALLBACK.take();
    let mut data = DATA.take();
    if let Some(func) = &mut func {
        func(&mut data);
    }
    CALLBACK.set(func);
    DATA.set(data);

    let data = DATA.take();
    let p = data.as_ptr();
    DATA.set(data);
    p
}

pub struct WebAudioDevice {}

impl BaseAudioOutputDevice for WebAudioDevice {}

unsafe impl Send for WebAudioDevice {}

impl Drop for WebAudioDevice {
    fn drop(&mut self) {
        unsafe {
            close_audio_output();
        }
    }
}

impl AudioOutputDevice for WebAudioDevice {
    fn new<C>(
        params: OutputDeviceParameters,
        data_callback: C,
    ) -> Result<WebAudioDevice, Box<dyn Error>>
    where
        C: FnMut(&mut [f32]) + Send + 'static,
    {
        DATA.set(vec![
            0.0;
            params.channel_sample_count * params.channels_count
        ]);
        CALLBACK.set(Some(Box::new(data_callback)));

        unsafe {
            let res = init_audio_output(
                params.sample_rate as u32,
                params.channels_count as u32,
                params.channel_sample_count as u32,
            );
            match res {
                0 => Ok(WebAudioDevice {}),
                -1 => Err("Failed to create whinyaudio context".into()),
                -2 => Err("Failed to create buffer when initialising whinyaudio".into()),
                _ => Err("Failed to setup whinyaudio (unrecognised error)".into()),
            }
        }
    }
}
