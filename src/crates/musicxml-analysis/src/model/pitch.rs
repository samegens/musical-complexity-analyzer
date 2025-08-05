#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoteName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Accidental {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pitch {
    pub note_name: NoteName,
    pub octave: u8,
    pub accidental: Accidental,
}

impl Pitch {
    pub fn new(note_name: NoteName, octave: u8, accidental: Accidental) -> Self {
        Self {
            note_name,
            octave,
            accidental,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pitch_creation() {
        // Act
        let pitch = Pitch::new(NoteName::C, 4, Accidental::Flat);

        // Assert
        assert_eq!(pitch.note_name, NoteName::C);
        assert_eq!(pitch.octave, 4);
        assert_eq!(pitch.accidental, Accidental::Flat);
    }

    #[test]
    fn test_pitch_equality() {
        // Arrange
        let pitch1 = Pitch::new(NoteName::C, 4, Accidental::Natural);
        let pitch2 = Pitch::new(NoteName::C, 4, Accidental::Natural);
        let pitch3 = Pitch::new(NoteName::D, 4, Accidental::Natural);

        // Assert
        assert_eq!(pitch1, pitch2);
        assert_ne!(pitch1, pitch3);
    }

    #[test]
    fn test_pitch_equality_with_unequal_accidentals() {
        // Arrange
        let pitch1 = Pitch::new(NoteName::C, 4, Accidental::Natural);
        let pitch2 = Pitch::new(NoteName::C, 4, Accidental::Sharp);

        // Act
        let result = pitch1.eq(&pitch2);

        // Assert
        assert!(!result);
    }
}
