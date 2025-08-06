use super::DensityMetrics;
use crate::model::MeasureData;

pub fn calculate_density_metrics(measure_data: &[MeasureData]) -> DensityMetrics {
    if measure_data.is_empty() {
        return DensityMetrics {
            average_notes_per_second: 0.0,
            peak_notes_per_second: 0.0,
            peak_measure: 0,
        };
    }

    let mut total_notes = 0;
    let mut total_duration_seconds = 0.0;
    let mut peak_notes_per_second = 0.0;
    let mut peak_measure: u32 = 0;

    for (measure_index, data) in measure_data.iter().enumerate() {
        total_notes += data.note_count;
        total_duration_seconds += data.get_measure_duration_seconds();

        let notes_per_second = data.note_count as f64 / data.get_measure_duration_seconds();
        if notes_per_second > peak_notes_per_second {
            peak_notes_per_second = notes_per_second;
            peak_measure = (measure_index + 1) as u32;
        }
    }

    let average_notes_per_second = total_notes as f64 / total_duration_seconds;

    DensityMetrics {
        average_notes_per_second,
        peak_notes_per_second,
        peak_measure,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use assert_float_eq::assert_float_absolute_eq;

    use crate::model::TimeSignature;

    use super::*;

    #[test]
    fn test_calculate_density_metrics_empty_data() {
        // Arrange
        let measure_data = vec![];

        // Act
        let metrics = calculate_density_metrics(&measure_data);

        // Assert
        assert_float_absolute_eq!(metrics.average_notes_per_second, 0.0, 0.001);
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 0.0, 0.001);
    }

    #[test]
    fn test_calculate_density_metrics_zero_notes() {
        // Arrange
        let measure_data = vec![MeasureData {
            note_count: 0,
            tempo_bpm: 120.0,
            time_signature: TimeSignature::new(4, 4),
            pitches: HashSet::new(),
        }];

        // Act
        let metrics = calculate_density_metrics(&measure_data);

        // Assert
        assert_float_absolute_eq!(metrics.average_notes_per_second, 0.0);
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 0.0);
        assert_eq!(metrics.peak_measure, 0);
    }

    #[test]
    fn test_calculate_density_metrics_single_measure() {
        // Arrange
        let measure_data = vec![MeasureData {
            note_count: 2,
            tempo_bpm: 120.0,
            time_signature: TimeSignature::new(4, 4),
            pitches: HashSet::new(),
        }];

        // Act
        let metrics: DensityMetrics = calculate_density_metrics(&measure_data);

        // Assert
        // 120 BPM = 0.5 sec/beat, 4/4 = 2 sec/measure
        // 2 notes in 2 seconds = 1.0 notes/sec
        assert_float_absolute_eq!(metrics.average_notes_per_second, 1.0);
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 1.0);
        assert_eq!(metrics.peak_measure, 1);
    }

    #[test]
    fn test_calculate_density_metrics_with_tempo_change() {
        // Arrange
        let measure_data = vec![
            MeasureData {
                note_count: 1,
                tempo_bpm: 120.0,
                time_signature: TimeSignature::new(4, 4),
                pitches: HashSet::new(),
            },
            MeasureData {
                note_count: 2,
                tempo_bpm: 60.0,
                time_signature: TimeSignature::new(4, 4),
                pitches: HashSet::new(),
            },
        ];

        // Act
        let metrics = calculate_density_metrics(&measure_data);

        // Assert
        // Measure 1: 120 BPM, 1 quarter note = 1 note in 2 seconds = 0.5 notes/sec
        // Measure 2: 60 BPM, 2 quarter notes = 2 notes in 4 seconds = 0.5 notes/sec
        // Average: 3 notes in 6 seconds = 0.5 notes/sec
        // Peak: 0.5 notes/sec (same for both measures)
        assert_float_absolute_eq!(metrics.average_notes_per_second, 0.5);
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 0.5);
        assert_eq!(metrics.peak_measure, 1);
    }

    #[test]
    fn test_calculate_density_metrics_with_time_signature_change() {
        // Arrange
        let measure_data = vec![
            MeasureData {
                note_count: 1,
                tempo_bpm: 120.0,
                time_signature: TimeSignature::new(4, 4),
                pitches: HashSet::new(),
            },
            MeasureData {
                note_count: 1,
                tempo_bpm: 120.0,
                time_signature: TimeSignature::new(3, 4),
                pitches: HashSet::new(),
            },
        ];

        // Act
        let metrics = calculate_density_metrics(&measure_data);

        // Assert
        // Measure 1: 4/4 time, 1 quarter note = 1 note in 2 seconds = 0.5 notes/sec
        // Measure 2: 3/4 time, 1 quarter note = 1 note in 1.5 seconds = 0.667 notes/sec
        // Average: 2 notes in 3.5 seconds = 0.571 notes/sec
        // Peak: 0.667 notes/sec (second measure)
        assert_float_absolute_eq!(metrics.average_notes_per_second, 2.0 / 3.5);
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 1.0 / 1.5);
        assert_eq!(metrics.peak_measure, 2);
    }
}
