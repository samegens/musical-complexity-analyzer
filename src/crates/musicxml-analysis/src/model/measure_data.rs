use std::collections::HashSet;

use crate::model::Pitch;

use super::TimeSignature;

#[derive(Debug, PartialEq)]
pub struct MeasureData {
    pub note_count: u32,
    pub tempo_bpm: f64,
    pub time_signature: TimeSignature,
    pub pitches: HashSet<Pitch>,
    pub left_hand_keys: Vec<u8>,  // Upper piano keys for left hand this measure
    pub right_hand_keys: Vec<u8>, // Upper piano keys for right hand this measure
}

impl MeasureData {
    pub fn new(note_count: u32, tempo_bpm: f64, time_signature: TimeSignature, pitches: HashSet<Pitch>) -> Self {
        Self {
            note_count,
            tempo_bpm,
            time_signature,
            pitches,
            left_hand_keys: Vec::new(),
            right_hand_keys: Vec::new(),
        }
    }
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
        let measure_data = MeasureData::new(
            4,
            120.0,
            TimeSignature::new(4, 4),
            HashSet::new(),
        );

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
