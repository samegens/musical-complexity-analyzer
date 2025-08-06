use std::collections::HashSet;

use crate::model::{MeasureData, Pitch};

use super::DiversityMetrics;

fn calculate_diversity_metrics(measure_data: &[MeasureData]) -> DiversityMetrics {
    let mut all_pitches: HashSet<Pitch> = HashSet::new();

    for data in measure_data {
        all_pitches.extend(&data.pitches);
    }

    DiversityMetrics {
        total_unique_pitches: all_pitches.len() as u32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Accidental, MeasureData, NoteName, Pitch, TimeSignature};

    #[test]
    fn test_calculate_diversity_metrics_empty_data() {
        // Arrange
        let measure_data = vec![];

        // Act
        let actual = calculate_diversity_metrics(&measure_data);

        // Assert
        let expected = DiversityMetrics {
            total_unique_pitches: 0,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_diversity_metrics_single_measure_with_unique_pitches() {
        // Arrange
        let pitches = HashSet::from([
            Pitch::new(NoteName::C, 4, Accidental::Natural),
            Pitch::new(NoteName::D, 4, Accidental::Natural),
            Pitch::new(NoteName::E, 4, Accidental::Natural),
        ]);

        let measure_data = vec![MeasureData {
            note_count: 3,
            pitches,
            tempo_bpm: 120.0,
            time_signature: TimeSignature::new(4, 4),
        }];

        // Act
        let actual = calculate_diversity_metrics(&measure_data);

        // Assert
        let expected = DiversityMetrics {
            total_unique_pitches: 3,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_diversity_metrics_multiple_measures_same_note() {
        // Arrange
        let c4_pitch = Pitch::new(NoteName::C, 4, Accidental::Natural);

        let measure_data = vec![
            MeasureData {
                note_count: 1,
                pitches: HashSet::from([c4_pitch]),
                tempo_bpm: 120.0,
                time_signature: TimeSignature::new(4, 4),
            },
            MeasureData {
                note_count: 1,
                pitches: HashSet::from([c4_pitch]),
                tempo_bpm: 120.0,
                time_signature: TimeSignature::new(4, 4),
            },
        ];

        // Act
        let actual = calculate_diversity_metrics(&measure_data);

        // Assert
        let expected = DiversityMetrics {
            total_unique_pitches: 1,
        };
        assert_eq!(actual, expected);
    }
}
