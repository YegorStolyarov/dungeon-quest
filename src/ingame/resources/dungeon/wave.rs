pub struct Wave {
    pub wave_number: usize,
    pub wave_duration: usize,
}

impl Wave {
    pub fn new() -> Self {
        Wave {
            wave_number: 1,
            wave_duration: 30,
        }
    }

    pub fn next_wave(&mut self) {
        self.wave_number += 1;
        self.wave_duration += 30;
    }
}
