use rodio::Source;
use std::time::Duration;

struct BaseOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl BaseOscillator {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>) -> BaseOscillator {
        BaseOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        sample
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index]
    }
}

impl Iterator for BaseOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}

pub struct WavetableOscillator {
    channels: u16,
    sample_rate: u32,
    oscillators: Vec<BaseOscillator>,
}

impl WavetableOscillator {
    pub fn new(
        channels: u16,
        sample_rate: u32,
        oscillators: Vec<(Vec<f32>, f32)>,
    ) -> WavetableOscillator {
        WavetableOscillator {
            channels,
            sample_rate,
            oscillators: oscillators
                .iter()
                .map(|(wt, frequency)| {
                    let mut osc = BaseOscillator::new(sample_rate, wt.clone());
                    osc.set_frequency(frequency.clone());
                    osc
                })
                .collect(),
        }
    }

    fn get_sample(&mut self) -> f32 {
        self.oscillators.iter_mut().map(|o| o.get_sample()).sum()
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate as u32
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
