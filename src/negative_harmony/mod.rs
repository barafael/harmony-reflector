pub use pitch_calc::{Letter, Step};

use std::collections::BTreeSet;

pub type Harmony = BTreeSet<Letter>;
pub type Chord = Vec<Letter>;

pub fn negate_note(note: Letter, root: Letter) -> Letter {
    let minor_third = root + Step::from(3_f32).letter();
    let major_third = root + Step::from(4_f32).letter();
    let distance;
    if note < minor_third {
        distance = minor_third - note;
        major_third + distance
    } else {
        distance = note - major_third;
        minor_third - distance
    }
}

pub fn negate_chord(chord: &[Letter], root: Letter) -> Chord {
    let mut negated = Chord::new();
    for note in chord {
        negated.push(negate_note(*note, root));
    }
    negated
}

pub fn negate_harmony(harmony: &Harmony, root: Letter) -> Harmony {
    let mut negated = Harmony::new();
    for note in harmony {
        negated.insert(negate_note(*note, root));
    }
    negated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g7_to_fmi6() {
        let mut g7 = Harmony::new();
        g7.insert(Letter::G);
        g7.insert(Letter::B);
        g7.insert(Letter::D);
        g7.insert(Letter::F);

        let negated = negate_harmony(&g7, Letter::C);

        let mut fmi6 = Harmony::new();
        fmi6.insert(Letter::F);
        fmi6.insert(Letter::Ab);
        fmi6.insert(Letter::C);
        fmi6.insert(Letter::D);
        assert_eq!(negated, fmi6);
    }

    #[test]
    fn cmaj_to_cmin() {
        let mut cmaj = Harmony::new();
        cmaj.insert(Letter::C);
        cmaj.insert(Letter::E);
        cmaj.insert(Letter::G);
        cmaj.insert(Letter::D);

        let negated = negate_harmony(&cmaj, Letter::C);

        let mut cmi11 = Harmony::new();
        cmi11.insert(Letter::C);
        cmi11.insert(Letter::Eb);
        cmi11.insert(Letter::F);
        cmi11.insert(Letter::G);
        assert_eq!(negated, cmi11);
    }

    #[test]
    fn chord_negation() {
        let mut dmi = Chord::new();
        dmi.push(Letter::D);
        dmi.push(Letter::A);
        dmi.push(Letter::E);
        dmi.push(Letter::F);
        dmi.push(Letter::C);

        let negated = negate_chord(&dmi, Letter::F);

        let expected = vec![Letter::Eb, Letter::Ab, Letter::Db, Letter::C, Letter::F];
        assert_eq!(expected, negated);
    }
}
