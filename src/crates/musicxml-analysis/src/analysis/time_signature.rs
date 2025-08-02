#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeSignature {
    pub numerator: u32,   // 3 in "3/4"
    pub denominator: u32, // 4 in "3/4"
}

impl TimeSignature {
    pub fn new(numerator: u32, denominator: u32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn beats_per_measure(&self) -> u32 {
        self.numerator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_signature_beats_per_measure() {
        // Arrange
        let sut = TimeSignature::new(3, 4);

        // Act
        let actual = sut.beats_per_measure();

        // Assert
        let expected = 3;
        assert_eq!(actual, expected);
    }
}
