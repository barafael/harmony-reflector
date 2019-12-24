use rodio::Source;
use std::time::Duration;

/// An infinite source that produces sine waves
///
/// Always has a rate of 48kHz and one channel.
#[derive(Clone, Debug)]
pub struct SineWaves {
    freqs: Vec<f32>,
    num_sample: usize,
}

impl SineWaves {
    /// The frequency of the sine.
    #[inline]
    pub fn new(freqs: Vec<f32>) -> SineWaves {
        SineWaves {
            freqs,
            num_sample: 0,
        }
    }
}

impl Iterator for SineWaves {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let mut values = Vec::new();
        let len = self.freqs.len() as f32;
        for freq in &self.freqs {
            let value = 2.0 * std::f32::consts::PI * *freq * self.num_sample as f32 / 48000.0;
            values.push(value);
        }
        let sum: f32 = values.iter().map(|s| s.sin() / len).sum();
        Some(sum)
    }
}

impl Source for SineWaves {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
