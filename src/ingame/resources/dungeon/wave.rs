use bevy::prelude::*;
use std::time::Duration;

pub struct Wave {
    pub wave_number: usize,
    pub wave_duration: u64,
    pub timer: Timer,
}

impl Wave {
    pub fn new() -> Self {
        let wave_duration: u64 = 5;
        let timer = Timer::new(Duration::from_secs(wave_duration), false);

        Wave {
            wave_number: 1,
            wave_duration,
            timer,
        }
    }

    pub fn next_wave(&mut self) {
        self.wave_number += 1;
        self.wave_duration += 5;
        self.timer = Timer::new(Duration::from_secs(self.wave_duration), false);
    }
}
