use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};

use sdl2::Sdl;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct SoundManager {
    audio_device: AudioDevice<SquareWave>,
}

impl SoundManager {
    pub fn new(context: &Sdl) -> SoundManager {
        let subsystem = context.audio().unwrap();
        let desired_spec =  AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None, 
        };

        let device = subsystem.open_playback(None, &desired_spec, |spec| {
            SquareWave {
                phase_inc: 240.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();

        SoundManager {
            audio_device: device, 
        }
    }

    pub fn play_sound(&mut self) {
        self.audio_device.resume();
    }

    pub fn stop_sound(&mut self) {
        self.audio_device.pause();
    }
}