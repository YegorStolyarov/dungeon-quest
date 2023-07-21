use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct Wave {
    pub wave_number: usize,
    pub wave_duration: i64,
    pub timer: Timer,
}

impl Wave {
    pub fn new() -> Self {
        let wave_duration: i64 = 15;
        let timer = Timer::new(Duration::from_secs(wave_duration as u64), TimerMode::Once);

        Wave {
            wave_number: 1,
            wave_duration,
            timer,
        }
    }

    pub fn next_wave(&mut self) {
        self.wave_number += 1;
        self.wave_duration += 15;
        self.timer = Timer::new(Duration::from_secs(self.wave_duration as u64), TimerMode::Once);
    }
}
