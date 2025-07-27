use musicxml::elements::{MeasureElement, PartElement, ScorePartwise};

#[derive(Debug, PartialEq)]
pub struct DensityMetrics {
    pub average_notes_per_second: f64,
    pub peak_notes_per_second: f64,
}

pub fn calculate_note_density(total_notes: u32, duration_seconds: f64) -> f64 {
    total_notes as f64 / duration_seconds
}

pub fn analyze_note_density(score: &ScorePartwise) -> DensityMetrics {
    const DEFAULT_BPM: f64 = 120.0;
    const DEFAULT_BEATS_PER_MEASURE: f64 = 4.0; // 4/4 time

    // Calculate measure duration in seconds
    let seconds_per_beat = 60.0 / DEFAULT_BPM;
    let seconds_per_measure = seconds_per_beat * DEFAULT_BEATS_PER_MEASURE;

    let mut total_notes = 0;
    let mut total_measures = 0;

    for part in &score.content.part {
        for part_element in &part.content {
            if let PartElement::Measure(measure) = part_element {
                total_measures += 1;

                total_notes += get_nr_notes_in_measure(measure);
            }
        }
    }

    let total_duration_seconds = total_measures as f64 * seconds_per_measure;
    let average_density = if total_duration_seconds > 0.0 {
        total_notes as f64 / total_duration_seconds
    } else {
        0.0
    };

    // For now, peak = average (single measure)
    let peak_density = average_density;

    DensityMetrics {
        average_notes_per_second: average_density,
        peak_notes_per_second: peak_density,
    }
}

fn get_nr_notes_in_measure(measure: &musicxml::elements::Measure) -> i32 {
    let mut nr_notes = 0;
    for measure_content in &measure.content {
        if let MeasureElement::Note(_) = measure_content {
            nr_notes += 1;
        }
    }
    nr_notes
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_absolute_eq;

    use super::*;

    #[test]
    fn test_note_density_calculation() {
        // Arrange
        let total_notes = 4;
        let duration_seconds = 8.0;

        // Act
        let actual = calculate_note_density(total_notes, duration_seconds);

        // Assert
        let expected = 0.5;
        assert_float_absolute_eq!(actual, expected);
    }

    #[test]
    fn test_analyze_note_density_empty_score_returns_zero() {
        // Arrange
        let score = create_empty_musicxml_dom();

        // Act
        let metrics = analyze_note_density(&score);

        // Assert
        assert_float_absolute_eq!(metrics.average_notes_per_second, 0.0, 0.001);
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 0.0, 0.001);
    }

    #[test]
    fn test_analyze_note_density_two_quarter_notes() {
        // Arrange
        let score = create_musicxml_dom_with_two_quarter_notes();

        // Act
        let metrics = analyze_note_density(&score);

        // Assert
        assert_float_absolute_eq!(metrics.average_notes_per_second, 1.0, 0.001);
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 1.0, 0.001);
    }

    fn create_empty_musicxml_dom() -> ScorePartwise {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE score-partwise PUBLIC "-//Recordare//DTD MusicXML 4.0 Partwise//EN" "http://www.musicxml.org/dtds/partwise.dtd">
<score-partwise version="4.0">
<part-list>
<score-part id="P1">
<part-name>Test</part-name>
</score-part>
</part-list>
<part id="P1">
</part>
</score-partwise>"#;

        parse_musicxml_to_dom(xml)
    }

    fn create_musicxml_dom_with_two_quarter_notes() -> ScorePartwise {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE score-partwise PUBLIC "-//Recordare//DTD MusicXML 4.0 Partwise//EN" "http://www.musicxml.org/dtds/partwise.dtd">
<score-partwise version="4.0">
<part-list>
<score-part id="P1">
<part-name>Test</part-name>
</score-part>
</part-list>
<part id="P1">
<measure number="1">
<note>
<pitch>
<step>C</step>
<octave>4</octave>
</pitch>
<duration>1</duration>
<type>quarter</type>
</note>
<note>
<pitch>
<step>D</step>
<octave>4</octave>
</pitch>
<duration>1</duration>
<type>quarter</type>
</note>
</measure>
</part>
</score-partwise>"#;

        parse_musicxml_to_dom(xml)
    }

    fn parse_musicxml_to_dom(xml: &str) -> ScorePartwise {
        musicxml::read_score_data_partwise(xml.as_bytes().to_vec())
            .expect("Failed to parse test XML")
    }
}
