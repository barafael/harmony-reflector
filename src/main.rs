extern crate rodio;

mod negative_harmony;
use crate::negative_harmony::*;

use rodio::{source::SineWave, Sink, Source};

use std::ops::Sub;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Hertz(f64);

impl Sub for Hertz {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl From<Hertz> for f64 {
    fn from(h: Hertz) -> Self {
        h.0
    }
}

impl From<f64> for Hertz {
    fn from(f: f64) -> Self {
        Self(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pitch(Hertz);

pub const STANDARD_PITCH: Hertz = Hertz(440.0);
pub const MIDDLE_C: Hertz = Hertz(261.626);

impl Pitch {
    pub fn new(frequency: Hertz) -> Self {
        Self(frequency)
    }
}

impl Default for Pitch {
    fn default() -> Self {
        Self(STANDARD_PITCH)
    }
}

impl From<Pitch> for f64 {
    fn from(p: Pitch) -> Self {
        p.0.into()
    }
}

impl From<Pitch> for SineWave {
    fn from(p: Pitch) -> Self {
        SineWave::new(f64::from(p) as u32)
    }
}

fn main() {
    let mut cmaj9 = negative_harmony::Harmony::new();
    cmaj9.insert(Letter::C);
    cmaj9.insert(Letter::E);
    cmaj9.insert(Letter::G);
    cmaj9.insert(Letter::D);

    let mut g7 = negative_harmony::Harmony::new();
    g7.insert(Letter::G);
    g7.insert(Letter::B);
    g7.insert(Letter::D);
    g7.insert(Letter::F);

    let negated = negative_harmony::negate_harmony(&cmaj9, Letter::C);

    let device = rodio::default_output_device().unwrap();

    let sink = Sink::new(&device);

    let mut sources = Vec::new();

    for note in negated {
        let freq = f64::from(pitch_calc::calc::hz_from_letter_octave(note, 4));
        let source = SineWave::from(Pitch::new(Hertz::from(freq))).amplify(0.1);
        sources.push(source);
    }
    let mix = sources[0]
        .clone()
        .mix(sources[1].clone())
        .mix(sources[2].clone())
        .mix(sources[3].clone());

    sink.append(mix);
    sink.sleep_until_end();
}
