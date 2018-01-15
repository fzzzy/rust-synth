// This is the example code modified from rust-sdl2/src/sdl2/audio.rs
// It now generates a sine wave instead of a square wave

extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;
use std::f32::consts::PI;

struct SineWave {
    phase_inc: f32,
    phase: f32,
}

impl AudioCallback for SineWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a sin wave
        for x in out.iter_mut() {
            *x = f32::sin(self.phase);
            self.phase = self.phase + self.phase_inc;
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),  // mono
        samples: None       // default sample size
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // initialize the audio callback
        SineWave {
            phase_inc: (PI * 2.0 * 440.0) / spec.freq as f32,
            phase: 0.0,
        }
    }).unwrap();

    // Start playback
    device.resume();

    // Play for 1 second
    std::thread::sleep(Duration::from_millis(1000));
}
