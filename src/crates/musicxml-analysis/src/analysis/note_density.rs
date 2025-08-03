use musicxml::{
    datatypes::NoteTypeValue,
    elements::{Measure, MeasureElement, MetronomeContents, PartElement, ScorePartwise},
};

use super::MeasureData;
use super::TimeSignature;

#[derive(Debug, PartialEq)]
pub struct DensityMetrics {
    pub average_notes_per_second: f64,
    pub peak_notes_per_second: f64,
}

fn extract_measure_data(score: &ScorePartwise) -> Vec<MeasureData> {
    let mut measure_data = Vec::new();
    let mut current_bpm = extract_bpm_from_score(score);
    let mut current_time_sig = extract_time_signature_from_score(score);

    for part in &score.content.part {
        for part_element in &part.content {
            if let PartElement::Measure(measure) = part_element {
                if let Some(new_bpm) = extract_bpm_from_measure(measure) {
                    current_bpm = new_bpm;
                }

                if let Some(new_time_sig) = extract_time_signature_from_measure(measure) {
                    current_time_sig = new_time_sig;
                }

                let note_count = get_nr_notes_in_measure(measure);

                measure_data.push(MeasureData {
                    note_count,
                    tempo_bpm: current_bpm,
                    time_signature: current_time_sig,
                });
            }
        }
    }

    measure_data
}

fn calculate_density_metrics(measure_data: &[MeasureData]) -> DensityMetrics {
    if measure_data.is_empty() {
        return DensityMetrics {
            average_notes_per_second: 0.0,
            peak_notes_per_second: 0.0,
        };
    }

    let mut total_notes = 0;
    let mut total_duration_seconds = 0.0;
    let mut peak_notes_per_second = 0.0;

    for data in measure_data {
        total_notes += data.note_count;
        total_duration_seconds += data.get_measure_duration_seconds();

        let notes_per_second = data.note_count as f64 / data.get_measure_duration_seconds();
        if notes_per_second > peak_notes_per_second {
            peak_notes_per_second = notes_per_second;
        }
    }

    let average_notes_per_second = if total_duration_seconds > 0.0 {
        total_notes as f64 / total_duration_seconds
    } else {
        0.0
    };

    DensityMetrics {
        average_notes_per_second,
        peak_notes_per_second,
    }
}

pub fn analyze_note_density(score: &ScorePartwise) -> DensityMetrics {
    let mut current_bpm = extract_bpm_from_score(score);
    let mut current_time_sig = extract_time_signature_from_score(score);

    let mut total_notes = 0;
    let mut total_duration_seconds = 0.0;
    let mut peak_notes_per_second = 0.0;

    for part in &score.content.part {
        for part_element in &part.content {
            if let PartElement::Measure(measure) = part_element {
                if let Some(new_bpm) = extract_bpm_from_measure(measure) {
                    current_bpm = new_bpm;
                }

                if let Some(new_time_sig) = extract_time_signature_from_measure(measure) {
                    current_time_sig = new_time_sig;
                }

                let beats_per_measure = current_time_sig.beats_per_measure() as f64;
                let seconds_per_beat = 60.0 / current_bpm;
                let seconds_per_measure = seconds_per_beat * beats_per_measure;

                let notes_in_measure = get_nr_notes_in_measure(measure);
                total_notes += notes_in_measure;
                total_duration_seconds += seconds_per_measure;

                let measure_density = notes_in_measure as f64 / seconds_per_measure;
                if measure_density > peak_notes_per_second {
                    peak_notes_per_second = measure_density;
                }
            }
        }
    }

    let average_notes_per_second = if total_duration_seconds > 0.0 {
        total_notes as f64 / total_duration_seconds
    } else {
        0.0
    };

    DensityMetrics {
        average_notes_per_second,
        peak_notes_per_second,
    }
}

fn extract_bpm_from_score(score: &ScorePartwise) -> f64 {
    const DEFAULT_BPM: f64 = 120.0;

    if let Some(first_part) = score.content.part.first() {
        if let Some(PartElement::Measure(first_measure)) = first_part.content.first() {
            if let Some(tempo) = extract_bpm_from_measure(first_measure) {
                return tempo;
            }
        }
    }

    DEFAULT_BPM
}

fn extract_bpm_from_measure(measure: &Measure) -> Option<f64> {
    for measure_content in &measure.content {
        if let MeasureElement::Direction(direction) = measure_content {
            if let Some(sound) = &direction.content.sound {
                if let Some(tempo) = &sound.attributes.tempo {
                    return Some(**tempo);
                }
            }

            for direction_type in &direction.content.direction_type {
                if let musicxml::elements::DirectionTypeContents::Metronome(metronome) =
                    &direction_type.content
                {
                    return extract_bpm_from_metronome(metronome);
                }
            }
        }
    }
    None
}

fn extract_bpm_from_metronome(metronome: &musicxml::elements::Metronome) -> Option<f64> {
    match &metronome.content {
        MetronomeContents::BeatBased(beat_based) => extract_bpm_from_beat_based(beat_based),
        _ => {
            panic!("Unsupported metronome content type for BPM extraction");
        }
    }
}

fn extract_bpm_from_beat_based(beat_based: &musicxml::elements::BeatBased) -> Option<f64> {
    match &beat_based.beat_unit.content {
        NoteTypeValue::Quarter => match &beat_based.equals {
            musicxml::elements::BeatEquation::BPM(per_minute) => per_minute.content.parse().ok(),
            _ => {
                panic!("Unsupported beat equation for BPM extraction");
            }
        },
        _ => {
            panic!("Unsupported beat unit for BPM extraction");
        }
    }
}

fn extract_time_signature_from_score(score: &ScorePartwise) -> TimeSignature {
    if let Some(first_part) = score.content.part.first() {
        for part_element in &first_part.content {
            if let PartElement::Measure(measure) = part_element {
                if let Some(time_sig) = extract_time_signature_from_measure(measure) {
                    return time_sig;
                }
            }
        }
    }

    TimeSignature {
        numerator: 4,
        denominator: 4,
    }
}

fn extract_time_signature_from_measure(measure: &Measure) -> Option<TimeSignature> {
    for measure_content in &measure.content {
        if let MeasureElement::Attributes(attributes) = measure_content {
            if let Some(first_time) = attributes.content.time.first() {
                let numerator = first_time
                    .content
                    .beats
                    .first()
                    .unwrap()
                    .beats
                    .content
                    .parse()
                    .ok()?;
                let denominator = first_time
                    .content
                    .beats
                    .first()
                    .unwrap()
                    .beat_type
                    .content
                    .parse()
                    .ok()?;
                return Some(TimeSignature {
                    numerator,
                    denominator,
                });
            }
        }
    }

    None
}

fn get_nr_notes_in_measure(measure: &musicxml::elements::Measure) -> u32 {
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

    use super::super::MeasureData;

    use super::*;

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

    #[test]
    fn test_analyze_note_density_peak_detection() {
        // Arrange
        let score = create_musicxml_dom_with_two_measures_with_varying_density();

        // Act
        let metrics = analyze_note_density(&score);

        // Assert
        assert_float_absolute_eq!(metrics.average_notes_per_second, 1.25); // (1+4)/2 measures = 5 notes / 4 seconds
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 2.0); // 4 notes in 2 seconds
    }

    #[test]
    fn test_analyze_note_density_with_60_bpm_in_notation() {
        // Arrange
        let score = create_musicxml_dom_with_two_quarter_notes_60bpm_in_notation();

        // Act
        let metrics = analyze_note_density(&score);

        // Assert
        assert_float_absolute_eq!(metrics.average_notes_per_second, 0.5); // 2 notes in 4 seconds
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 0.5); // same for single measure
    }

    #[test]
    fn test_analyze_note_density_sound_tempo_overrides_metronome() {
        // Arrange
        let score =
            create_score_with_two_quarter_notes_60bpm_in_sound_tempo_and_120bpm_in_metronome();

        // Act
        let metrics = analyze_note_density(&score);

        // Assert
        let bpm = 60.0;
        let measures_per_minute = bpm / 4.0;
        let seconds_per_measure = 60.0 / measures_per_minute;
        assert_float_absolute_eq!(metrics.average_notes_per_second, 2.0 / seconds_per_measure);
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 0.5);
    }

    #[test]
    fn test_analyze_note_density_with_3_4_time_signature() {
        // Arrange
        let score = create_musicxml_dom_with_3_4_time_signature();

        // Act
        let metrics = analyze_note_density(&score);

        // Assert
        // With 3/4 time at 120 BPM:
        // - 3 beats per measure = 1.5 seconds per measure
        // - 3 quarter notes in one measure = 3 notes / 1.5 seconds = 2.0 notes per second
        assert_float_absolute_eq!(metrics.average_notes_per_second, 2.0);
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 2.0);
    }

    #[test]
    fn test_analyze_note_density_with_tempo_change() {
        // Arrange
        let score = create_musicxml_dom_with_tempo_change();

        // Act
        let metrics = analyze_note_density(&score);

        // Assert
        // Measure 1: 120 BPM, 1 quarter note = 1 note in 2 seconds = 0.5 notes/sec
        // Measure 2: 60 BPM, 2 quarter notes = 2 notes in 4 seconds = 0.5 notes/sec
        // Average: 3 notes in 6 seconds = 0.5 notes/sec
        // Peak: 0.5 notes/sec (same for both measures)
        assert_float_absolute_eq!(metrics.average_notes_per_second, 0.5);
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 0.5);
    }

    #[test]
    fn test_analyze_note_density_with_time_signature_change() {
        // Arrange
        let score = create_musicxml_dom_with_time_signature_change();

        // Act
        let metrics = analyze_note_density(&score);

        // Assert
        // Measure 1: 4/4 time, 1 quarter note = 1 note in 2 seconds = 0.5 notes/sec
        // Measure 2: 3/4 time, 1 quarter note = 1 note in 1.5 seconds = 0.667 notes/sec
        // Average: 2 notes in 3.5 seconds = 0.571 notes/sec
        // Peak: 0.667 notes/sec (second measure)
        assert_float_absolute_eq!(metrics.average_notes_per_second, 0.571, 0.001);
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 0.667, 0.001);
    }

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
        }];

        // Act
        let metrics = calculate_density_metrics(&measure_data);

        // Assert
        assert_float_absolute_eq!(metrics.average_notes_per_second, 0.0, 0.001);
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 0.0, 0.001);
    }

    #[test]
    fn test_calculate_density_metrics_single_measure() {
        // Arrange
        let measure_data = vec![MeasureData {
            note_count: 2,
            tempo_bpm: 120.0,
            time_signature: TimeSignature::new(4, 4),
        }];

        // Act
        let metrics: DensityMetrics = calculate_density_metrics(&measure_data);

        // Assert
        // 120 BPM = 0.5 sec/beat, 4/4 = 2 sec/measure
        // 2 notes in 2 seconds = 1.0 notes/sec
        assert_float_absolute_eq!(metrics.average_notes_per_second, 1.0);
        assert_float_absolute_eq!(metrics.peak_notes_per_second, 1.0);
    }

    #[test]
    fn test_calculate_density_metrics_with_tempo_change() {
        // Arrange
        let measure_data = vec![
            MeasureData {
                note_count: 1,
                tempo_bpm: 120.0,
                time_signature: TimeSignature::new(4, 4),
            },
            MeasureData {
                note_count: 2,
                tempo_bpm: 60.0,
                time_signature: TimeSignature::new(4, 4),
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
    }

    #[test]
    fn test_calculate_density_metrics_with_time_signature_change() {
        // Arrange
        let measure_data = vec![
            MeasureData {
                note_count: 1,
                tempo_bpm: 120.0,
                time_signature: TimeSignature::new(4, 4),
            },
            MeasureData {
                note_count: 1,
                tempo_bpm: 120.0,
                time_signature: TimeSignature::new(3, 4),
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
    }

    #[test]
    fn test_extract_measure_data_single_measure() {
        // Arrange
        let score = create_musicxml_dom_with_two_quarter_notes();

        // Act
        let measure_data = extract_measure_data(&score);

        // Assert
        assert_eq!(measure_data.len(), 1);
        assert_eq!(measure_data[0].note_count, 2);
        assert_eq!(measure_data[0].tempo_bpm, 120.0); // Default
        assert_eq!(measure_data[0].time_signature, TimeSignature::new(4, 4)); // Default
    }

    #[test]
    fn test_extract_measure_data_single_measure_defaults() {
        // Arrange
        let score = create_musicxml_dom_with_two_quarter_notes();

        // Act
        let result = extract_measure_data(&score);

        // Assert
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].note_count, 2);
        assert_eq!(result[0].tempo_bpm, 120.0);
        assert_eq!(result[0].time_signature, TimeSignature::new(4, 4));
    }

    #[test]
    fn test_extract_measure_data_tempo_change() {
        // Arrange
        let score = create_musicxml_dom_with_tempo_change();

        // Act
        let result = extract_measure_data(&score);

        // Assert
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].tempo_bpm, 120.0);
        assert_eq!(result[1].tempo_bpm, 60.0);
        // Note counts should match expected values
        assert_eq!(result[0].note_count, 1);
        assert_eq!(result[1].note_count, 2);
    }

    #[test]
    fn test_extract_measure_data_time_signature_change() {
        // Arrange
        let score = create_musicxml_dom_with_time_signature_change();

        // Act
        let result = extract_measure_data(&score);

        // Assert
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].time_signature, TimeSignature::new(4, 4));
        assert_eq!(result[1].time_signature, TimeSignature::new(3, 4));
        // Note counts should match expected values
        assert_eq!(result[0].note_count, 1);
        assert_eq!(result[1].note_count, 1);
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

    fn create_musicxml_dom_with_two_quarter_notes_60bpm_in_notation() -> ScorePartwise {
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
      <direction placement="above">
        <direction-type>
          <metronome>
            <beat-unit>quarter</beat-unit>
            <per-minute>60</per-minute>
          </metronome>
        </direction-type>
      </direction>
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

    fn create_score_with_two_quarter_notes_60bpm_in_sound_tempo_and_120bpm_in_metronome()
    -> ScorePartwise {
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
      <direction>
        <sound tempo="60"/>
        <direction-type>
          <metronome>
            <beat-unit>quarter</beat-unit>
            <per-minute>120</per-minute>
          </metronome>
        </direction-type>
      </direction>
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

    fn create_musicxml_dom_with_two_measures_with_varying_density() -> ScorePartwise {
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
   </measure>
   <measure number="2">
     <note>
       <pitch>
         <step>D</step>
         <octave>4</octave>
       </pitch>
       <duration>1</duration>
       <type>quarter</type>
     </note>
     <note>
       <pitch>
         <step>E</step>
         <octave>4</octave>
       </pitch>
       <duration>1</duration>
       <type>quarter</type>
     </note>
     <note>
       <pitch>
         <step>F</step>
         <octave>4</octave>
       </pitch>
       <duration>1</duration>
       <type>quarter</type>
     </note>
     <note>
       <pitch>
         <step>G</step>
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

    fn create_musicxml_dom_with_3_4_time_signature() -> ScorePartwise {
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
      <attributes>
        <time>
          <beats>3</beats>
          <beat-type>4</beat-type>
        </time>
      </attributes>
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>1</duration>
        <type>quarter</type>
      </note>
      <note>
        <pitch><step>D</step><octave>4</octave></pitch>
        <duration>1</duration>
        <type>quarter</type>
      </note>
      <note>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>1</duration>
        <type>quarter</type>
      </note>
    </measure>
  </part>
</score-partwise>"#;

        parse_musicxml_to_dom(xml)
    }

    fn create_musicxml_dom_with_tempo_change() -> ScorePartwise {
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
      <direction placement="above">
        <direction-type>
          <metronome>
            <beat-unit>quarter</beat-unit>
            <per-minute>120</per-minute>
          </metronome>
        </direction-type>
      </direction>
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>1</duration>
        <type>quarter</type>
      </note>
    </measure>
    <measure number="2">
      <direction placement="above">
        <direction-type>
          <metronome>
            <beat-unit>quarter</beat-unit>
            <per-minute>60</per-minute>
          </metronome>
        </direction-type>
      </direction>
      <note>
        <pitch><step>D</step><octave>4</octave></pitch>
        <duration>1</duration>
        <type>quarter</type>
      </note>
      <note>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>1</duration>
        <type>quarter</type>
      </note>
    </measure>
  </part>
</score-partwise>"#;

        parse_musicxml_to_dom(xml)
    }

    fn create_musicxml_dom_with_time_signature_change() -> ScorePartwise {
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
      <attributes>
        <time>
          <beats>4</beats>
          <beat-type>4</beat-type>
        </time>
      </attributes>
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>1</duration>
        <type>quarter</type>
      </note>
    </measure>
    <measure number="2">
      <attributes>
        <time>
          <beats>3</beats>
          <beat-type>4</beat-type>
        </time>
      </attributes>
      <note>
        <pitch><step>D</step><octave>4</octave></pitch>
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
