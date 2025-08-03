use super::TimeSignature;

#[derive(Debug, PartialEq)]
pub struct MeasureData {
    pub note_count: u32,
    pub tempo_bpm: f64,
    pub time_signature: TimeSignature,
}

impl MeasureData {
    pub fn get_measure_duration_seconds(&self) -> f64 {
        let seconds_per_beat = 60.0 / self.tempo_bpm;
        let beats_per_measure = self.time_signature.beats_per_measure() as f64;
        seconds_per_beat * beats_per_measure
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_absolute_eq;

    use super::*;

    #[test]
    fn test_get_measure_duration_seconds_standard_4_4_at_120_bpm() {
        // Arrange
        let measure_data = MeasureData {
            note_count: 4,
            tempo_bpm: 120.0,
            time_signature: TimeSignature::new(4, 4),
        };

        // Act
        let actual = measure_data.get_measure_duration_seconds();

        // Assert
        // 120 BPM = 0.5 seconds per beat
        // 4/4 time = 4 beats per measure
        // Duration = 0.5 * 4 = 2.0 seconds
        let expected = 2.0;
        assert_float_absolute_eq!(actual, expected);
    }
}
