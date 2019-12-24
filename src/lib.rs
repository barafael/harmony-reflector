use pitch_calc::{
    Hz,
    Letter,
    LetterOctave,
    ScaledPerc,
    Step,
};

use std::collections::BTreeSet;

type Harmony = BTreeSet<Letter>;
type Chord = Vec<Letter>;

fn negate_note(note: Letter, root: Letter) -> Letter {
    let minor_third = root + Step::from(3 as f32).letter();
    let major_third = root + Step::from(4 as f32).letter();
    let distance;
    if note < minor_third {
        distance = minor_third - note;
        return major_third + distance;
    } else {
        distance = note - major_third;
        return minor_third - distance;
    }
}

fn negate_chord(chord: &Chord, root: Letter) -> Chord {
    let mut negated = Chord::new();
    for note in chord {
        negated.push(negate_note(*note, root));
    }
    return negated;
}

fn negate_harmony(harmony: &Harmony, root: Letter) -> Harmony {
    let mut negated = Harmony::new();
    for note in harmony {
        negated.insert(negate_note(*note, root));
    }
    return negated;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

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
        fmi6.insert(Letter::Gsh);
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
        cmi11.insert(Letter::Dsh);
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

        let mut expected = vec![Letter::Dsh, Letter::Gsh, Letter::Csh, Letter::C, Letter::F];
        assert_eq!(expected, negated);
    }
}
