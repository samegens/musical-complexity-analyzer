use super::{Accidental, NoteName, Pitch};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChromaticNoteName {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PianoKey {
    pub note_name: ChromaticNoteName,
    pub octave: u8,
}

impl PianoKey {
    pub fn from_pitch(pitch: &Pitch) -> Self {
        let (chromatic_note, octave_adjustment) =
            convert_to_chromatic_note_with_octave(pitch.note_name, pitch.accidental);

        Self {
            note_name: chromatic_note,
            octave: (pitch.octave as i8 + octave_adjustment) as u8,
        }
    }
}

fn convert_to_chromatic_note_with_octave(
    note_name: NoteName,
    accidental: Accidental,
) -> (ChromaticNoteName, i8) {
    match (note_name, accidental) {
        (NoteName::C, Accidental::DoubleFlat) => (ChromaticNoteName::ASharp, -1),
        (NoteName::C, Accidental::Flat) => (ChromaticNoteName::B, -1),
        (NoteName::C, Accidental::Natural) => (ChromaticNoteName::C, 0),
        (NoteName::C, Accidental::Sharp) => (ChromaticNoteName::CSharp, 0),
        (NoteName::C, Accidental::DoubleSharp) => (ChromaticNoteName::D, 0),

        (NoteName::D, Accidental::DoubleFlat) => (ChromaticNoteName::C, 0),
        (NoteName::D, Accidental::Flat) => (ChromaticNoteName::CSharp, 0),
        (NoteName::D, Accidental::Natural) => (ChromaticNoteName::D, 0),
        (NoteName::D, Accidental::Sharp) => (ChromaticNoteName::DSharp, 0),
        (NoteName::D, Accidental::DoubleSharp) => (ChromaticNoteName::E, 0),

        (NoteName::E, Accidental::DoubleFlat) => (ChromaticNoteName::D, 0),
        (NoteName::E, Accidental::Flat) => (ChromaticNoteName::DSharp, 0),
        (NoteName::E, Accidental::Natural) => (ChromaticNoteName::E, 0),
        (NoteName::E, Accidental::Sharp) => (ChromaticNoteName::F, 0),
        (NoteName::E, Accidental::DoubleSharp) => (ChromaticNoteName::FSharp, 0),

        (NoteName::F, Accidental::DoubleFlat) => (ChromaticNoteName::DSharp, 0),
        (NoteName::F, Accidental::Flat) => (ChromaticNoteName::E, 0),
        (NoteName::F, Accidental::Natural) => (ChromaticNoteName::F, 0),
        (NoteName::F, Accidental::Sharp) => (ChromaticNoteName::FSharp, 0),
        (NoteName::F, Accidental::DoubleSharp) => (ChromaticNoteName::G, 0),

        (NoteName::G, Accidental::DoubleFlat) => (ChromaticNoteName::F, 0),
        (NoteName::G, Accidental::Flat) => (ChromaticNoteName::FSharp, 0),
        (NoteName::G, Accidental::Natural) => (ChromaticNoteName::G, 0),
        (NoteName::G, Accidental::Sharp) => (ChromaticNoteName::GSharp, 0),
        (NoteName::G, Accidental::DoubleSharp) => (ChromaticNoteName::A, 0),

        (NoteName::A, Accidental::DoubleFlat) => (ChromaticNoteName::G, 0),
        (NoteName::A, Accidental::Flat) => (ChromaticNoteName::GSharp, 0),
        (NoteName::A, Accidental::Natural) => (ChromaticNoteName::A, 0),
        (NoteName::A, Accidental::Sharp) => (ChromaticNoteName::ASharp, 0),
        (NoteName::A, Accidental::DoubleSharp) => (ChromaticNoteName::B, 0),

        (NoteName::B, Accidental::DoubleFlat) => (ChromaticNoteName::A, 0),
        (NoteName::B, Accidental::Flat) => (ChromaticNoteName::ASharp, 0),
        (NoteName::B, Accidental::Natural) => (ChromaticNoteName::B, 0),
        (NoteName::B, Accidental::Sharp) => (ChromaticNoteName::C, 1),
        (NoteName::B, Accidental::DoubleSharp) => (ChromaticNoteName::CSharp, 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // Basic naturals
    #[case(NoteName::C, 4, Accidental::Natural, ChromaticNoteName::C, 4)]
    #[case(NoteName::D, 4, Accidental::Natural, ChromaticNoteName::D, 4)]
    #[case(NoteName::E, 4, Accidental::Natural, ChromaticNoteName::E, 4)]
    #[case(NoteName::F, 4, Accidental::Natural, ChromaticNoteName::F, 4)]
    #[case(NoteName::G, 4, Accidental::Natural, ChromaticNoteName::G, 4)]
    #[case(NoteName::A, 4, Accidental::Natural, ChromaticNoteName::A, 4)]
    #[case(NoteName::B, 4, Accidental::Natural, ChromaticNoteName::B, 4)]
    // Basic sharps
    #[case(NoteName::C, 4, Accidental::Sharp, ChromaticNoteName::CSharp, 4)]
    #[case(NoteName::D, 4, Accidental::Sharp, ChromaticNoteName::DSharp, 4)]
    #[case(NoteName::F, 4, Accidental::Sharp, ChromaticNoteName::FSharp, 4)]
    #[case(NoteName::G, 4, Accidental::Sharp, ChromaticNoteName::GSharp, 4)]
    #[case(NoteName::A, 4, Accidental::Sharp, ChromaticNoteName::ASharp, 4)]
    // Basic flats
    #[case(NoteName::D, 4, Accidental::Flat, ChromaticNoteName::CSharp, 4)]
    #[case(NoteName::E, 4, Accidental::Flat, ChromaticNoteName::DSharp, 4)]
    #[case(NoteName::G, 4, Accidental::Flat, ChromaticNoteName::FSharp, 4)]
    #[case(NoteName::A, 4, Accidental::Flat, ChromaticNoteName::GSharp, 4)]
    #[case(NoteName::B, 4, Accidental::Flat, ChromaticNoteName::ASharp, 4)]
    // Octave boundary crossings
    #[case(NoteName::B, 4, Accidental::Sharp, ChromaticNoteName::C, 5)]
    #[case(NoteName::C, 4, Accidental::Flat, ChromaticNoteName::B, 3)]
    #[case(NoteName::E, 4, Accidental::Sharp, ChromaticNoteName::F, 4)]
    #[case(NoteName::F, 4, Accidental::Flat, ChromaticNoteName::E, 4)]
    // Double sharps
    #[case(NoteName::C, 4, Accidental::DoubleSharp, ChromaticNoteName::D, 4)]
    #[case(NoteName::D, 4, Accidental::DoubleSharp, ChromaticNoteName::E, 4)]
    #[case(NoteName::E, 4, Accidental::DoubleSharp, ChromaticNoteName::FSharp, 4)]
    #[case(NoteName::F, 4, Accidental::DoubleSharp, ChromaticNoteName::G, 4)]
    #[case(NoteName::G, 4, Accidental::DoubleSharp, ChromaticNoteName::A, 4)]
    #[case(NoteName::A, 4, Accidental::DoubleSharp, ChromaticNoteName::B, 4)]
    #[case(NoteName::B, 4, Accidental::DoubleSharp, ChromaticNoteName::CSharp, 5)]
    // Double flats
    #[case(NoteName::C, 4, Accidental::DoubleFlat, ChromaticNoteName::ASharp, 3)]
    #[case(NoteName::D, 4, Accidental::DoubleFlat, ChromaticNoteName::C, 4)]
    #[case(NoteName::E, 4, Accidental::DoubleFlat, ChromaticNoteName::D, 4)]
    #[case(NoteName::F, 4, Accidental::DoubleFlat, ChromaticNoteName::DSharp, 4)]
    #[case(NoteName::G, 4, Accidental::DoubleFlat, ChromaticNoteName::F, 4)]
    #[case(NoteName::A, 4, Accidental::DoubleFlat, ChromaticNoteName::G, 4)]
    #[case(NoteName::B, 4, Accidental::DoubleFlat, ChromaticNoteName::A, 4)]
    fn test_piano_key_conversions(
        #[case] note_name: NoteName,
        #[case] octave: u8,
        #[case] accidental: Accidental,
        #[case] expected_chromatic: ChromaticNoteName,
        #[case] expected_octave: u8,
    ) {
        // Arrange
        let pitch = Pitch::new(note_name, octave, accidental);

        // Act
        let piano_key = PianoKey::from_pitch(&pitch);

        // Assert
        assert_eq!(piano_key.note_name, expected_chromatic);
        assert_eq!(piano_key.octave, expected_octave);
    }

    #[rstest]
    // C# = Db
    #[case(vec![(NoteName::C, 4, Accidental::Sharp), (NoteName::D, 4, Accidental::Flat)])]
    // D = C## = Ebb
    #[case(vec![(NoteName::D, 4, Accidental::Natural), (NoteName::C, 4, Accidental::DoubleSharp), (NoteName::E, 4, Accidental::DoubleFlat)])]
    // F = E# = Gbb
    #[case(vec![(NoteName::F, 4, Accidental::Natural), (NoteName::E, 4, Accidental::Sharp), (NoteName::G, 4, Accidental::DoubleFlat)])]
    fn test_enharmonic_equivalents(#[case] enharmonic_group: Vec<(NoteName, u8, Accidental)>) {
        let piano_keys: Vec<PianoKey> = enharmonic_group
            .iter()
            .map(|(note, octave, acc)| PianoKey::from_pitch(&Pitch::new(*note, *octave, *acc)))
            .collect();

        for i in 1..piano_keys.len() {
            assert_eq!(piano_keys[0], piano_keys[i]);
        }
    }
}
