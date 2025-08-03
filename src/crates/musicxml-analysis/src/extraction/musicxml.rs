use musicxml::{
    datatypes::NoteTypeValue,
    elements::{Measure, MeasureElement, MetronomeContents, PartElement, ScorePartwise},
};

use crate::model::{MeasureData, TimeSignature};

pub fn extract_measure_data(score: &ScorePartwise) -> Vec<MeasureData> {
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
    use super::*;

    #[test]
    fn test_extract_measure_data_returns_empty_vec_when_no_measures() {
        // Arrange
        let score = create_empty_musicxml_dom();

        // Act
        let actual_measure_data = extract_measure_data(&score);

        // Assert
        assert!(actual_measure_data.is_empty());
    }

    #[test]
    fn test_extract_measure_data_single_measure() {
        // Arrange
        let score = create_musicxml_dom_with_two_quarter_notes();

        // Act
        let actual = extract_measure_data(&score);

        // Assert
        let expected = vec![MeasureData {
            note_count: 2,
            tempo_bpm: 120.0,                         // Default
            time_signature: TimeSignature::new(4, 4), // Default
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_extract_measure_data_two_measures() {
        // Arrange
        let score = create_musicxml_dom_with_two_measures();

        // Act
        let actual = extract_measure_data(&score);

        // Assert
        let expected = vec![
            MeasureData {
                note_count: 1,
                tempo_bpm: 120.0,
                time_signature: TimeSignature::new(4, 4),
            },
            MeasureData {
                note_count: 2,
                tempo_bpm: 120.0,
                time_signature: TimeSignature::new(4, 4),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_extract_measure_data_with_metronome_direction() {
        // Arrange
        let score = create_musicxml_dom_with_60bpm_in_notation();

        // Act
        let actual = extract_measure_data(&score);

        // Assert
        let expected = vec![MeasureData {
            note_count: 0,
            tempo_bpm: 60.0,
            time_signature: TimeSignature::new(4, 4),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_extract_measure_data_with_metronome_120_bpm_and_tempo_60bpm() {
        // Arrange
        let score = create_score_with_60bpm_in_sound_tempo_and_120bpm_in_metronome();

        // Act
        let actual = extract_measure_data(&score);

        // Assert
        let expected = vec![MeasureData {
            note_count: 0,
            tempo_bpm: 60.0,
            time_signature: TimeSignature::new(4, 4),
        }];
        assert_eq!(actual, expected);
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

    fn create_musicxml_dom_with_60bpm_in_notation() -> ScorePartwise {
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
    </measure>
  </part>
</score-partwise>"#;

        parse_musicxml_to_dom(xml)
    }

    fn create_score_with_60bpm_in_sound_tempo_and_120bpm_in_metronome() -> ScorePartwise {
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
    </measure>
  </part>
</score-partwise>"#;

        parse_musicxml_to_dom(xml)
    }

    fn create_musicxml_dom_with_two_measures() -> ScorePartwise {
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
    </measure>
    <measure number="2">
      <attributes>
        <time>
          <beats>3</beats>
          <beat-type>4</beat-type>
        </time>
      </attributes>
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
