use std::collections::HashSet;

use musicxml::{
    datatypes::NoteTypeValue,
    elements::{
        AudibleType, Measure, MeasureElement, MetronomeContents, NoteType, PartElement,
        ScorePartwise,
    },
};

use crate::model::{MeasureData, NoteName, Pitch, TimeSignature, pitch::Accidental};

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
                let pitches = extract_pitches_from_measure(measure);

                measure_data.push(MeasureData {
                    note_count,
                    tempo_bpm: current_bpm,
                    time_signature: current_time_sig,
                    pitches,
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
    let per_minute = match &beat_based.equals {
        musicxml::elements::BeatEquation::BPM(per_minute) => per_minute.content.parse().ok()?,
        _ => {
            panic!(
                "Unsupported beat equation for BPM extraction: {:?}",
                beat_based.equals
            );
        }
    };

    let quarter_note_bpm = match &beat_based.beat_unit.content {
        NoteTypeValue::Quarter => per_minute,
        NoteTypeValue::Half => per_minute * 2.0,
        _ => {
            panic!(
                "Unsupported beat unit for BPM extraction: {:?}",
                beat_based.beat_unit.content
            );
        }
    };

    if beat_based.beat_unit_dot.is_empty() {
        Some(quarter_note_bpm)
    } else {
        Some(quarter_note_bpm * 1.5)
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
        if let MeasureElement::Note(note) = measure_content {
            if let NoteType::Normal(normal_info) = &note.content.info {
                if let AudibleType::Pitch(_) = &normal_info.audible {
                    nr_notes += 1;
                }
            }
        }
    }
    nr_notes
}

fn extract_pitches_from_measure(measure: &Measure) -> HashSet<Pitch> {
    let mut pitches = HashSet::new();

    for measure_content in &measure.content {
        if let MeasureElement::Note(note) = measure_content {
            if let NoteType::Normal(normal_info) = &note.content.info {
                if let AudibleType::Pitch(pitch_info) = &normal_info.audible {
                    let note_name = extract_note_name_from_pitch(pitch_info);
                    let octave = *pitch_info.content.octave.content;
                    let accidental = get_accidental_from_pitch(pitch_info);
                    pitches.insert(Pitch::new(note_name, octave, accidental));
                }
            }
        }
    }

    pitches
}

fn extract_note_name_from_pitch(musicxml_pitch: &musicxml::elements::Pitch) -> NoteName {
    match musicxml_pitch.content.step.content {
        musicxml::datatypes::Step::A => NoteName::A,
        musicxml::datatypes::Step::B => NoteName::B,
        musicxml::datatypes::Step::C => NoteName::C,
        musicxml::datatypes::Step::D => NoteName::D,
        musicxml::datatypes::Step::E => NoteName::E,
        musicxml::datatypes::Step::F => NoteName::F,
        musicxml::datatypes::Step::G => NoteName::G,
    }
}

fn get_accidental_from_pitch(musicxml_pitch: &musicxml::elements::Pitch) -> Accidental {
    if let Some(alter) = &musicxml_pitch.content.alter {
        match *alter.content {
            -2 => Accidental::DoubleFlat,
            -1 => Accidental::Flat,
            0 => Accidental::Natural,
            1 => Accidental::Sharp,
            2 => Accidental::DoubleSharp,
            _ => panic!("Unsupported alter {}", *alter.content),
        }
    } else {
        Accidental::Natural
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{NoteName, Pitch, pitch::Accidental};

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

    fn create_empty_musicxml_dom() -> ScorePartwise {
        create_test_score("")
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
            pitches: HashSet::from([
                Pitch::new(NoteName::C, 4, Accidental::Natural),
                Pitch::new(NoteName::D, 4, Accidental::Natural),
            ]),
        }];
        assert_eq!(actual, expected);
    }

    fn create_musicxml_dom_with_two_quarter_notes() -> ScorePartwise {
        let measures = format!(
            r#"<measure number="1">
    {}
    {}
</measure>"#,
            create_note("C", 4),
            create_note("D", 4)
        );
        create_test_score(&measures)
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
                pitches: HashSet::from([Pitch::new(NoteName::C, 4, Accidental::Natural)]),
            },
            MeasureData {
                note_count: 2,
                tempo_bpm: 120.0,
                time_signature: TimeSignature::new(4, 4),
                pitches: HashSet::from([
                    Pitch::new(NoteName::D, 4, Accidental::Natural),
                    Pitch::new(NoteName::E, 4, Accidental::Natural),
                ]),
            },
        ];
        assert_eq!(actual, expected);
    }

    fn create_musicxml_dom_with_two_measures() -> ScorePartwise {
        let measures = format!(
            r#"<measure number="1">
    {note_c}
</measure>
<measure number="2">
    {note_d}
    {note_e}
</measure>"#,
            note_c = create_note("C", 4),
            note_d = create_note("D", 4),
            note_e = create_note("E", 4)
        );
        create_test_score(&measures)
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
            pitches: HashSet::new(),
        }];
        assert_eq!(actual, expected);
    }

    fn create_musicxml_dom_with_60bpm_in_notation() -> ScorePartwise {
        let measures = format!(
            r#"<measure number="1">
    {metronome}
</measure>"#,
            metronome = create_metronome("quarter", 60)
        );
        create_test_score(&measures)
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
            pitches: HashSet::new(),
        }];
        assert_eq!(actual, expected);
    }

    fn create_score_with_60bpm_in_sound_tempo_and_120bpm_in_metronome() -> ScorePartwise {
        let direction = r#"<direction>
    <sound tempo="60"/>
    <direction-type>
        <metronome>
            <beat-unit>quarter</beat-unit>
            <per-minute>120</per-minute>
        </metronome>
    </direction-type>
</direction>"#;

        let measures = format!(
            r#"<measure number="1">
    {direction}
</measure>"#
        );

        create_test_score(&measures)
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

    fn create_musicxml_dom_with_tempo_change() -> ScorePartwise {
        let measures = format!(
            r#"<measure number="1">
    {metronome_120}
</measure>
<measure number="2">
    {metronome_60}
</measure>"#,
            metronome_120 = create_metronome("quarter", 120),
            metronome_60 = create_metronome("quarter", 60)
        );
        create_test_score(&measures)
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

    fn create_musicxml_dom_with_time_signature_change() -> ScorePartwise {
        let measures = format!(
            r#"<measure number="1">
    {time_sig_4_4}
</measure>
<measure number="2">
    {time_sig_3_4}
</measure>"#,
            time_sig_4_4 = create_time_signature(4, 4),
            time_sig_3_4 = create_time_signature(3, 4)
        );
        create_test_score(&measures)
    }

    #[test]
    fn test_extract_measure_data_excludes_rests() {
        // Arrange
        let score = create_musicxml_dom_with_rest_only();

        // Act
        let actual = extract_measure_data(&score);

        // Assert
        let expected = vec![MeasureData {
            note_count: 0,
            tempo_bpm: 120.0,
            time_signature: TimeSignature::new(4, 4),
            pitches: HashSet::new(),
        }];
        assert_eq!(actual, expected);
    }

    fn create_musicxml_dom_with_rest_only() -> ScorePartwise {
        let measures = format!(
            r#"<measure number="1">
    {}
</measure>"#,
            create_rest()
        );
        create_test_score(&measures)
    }

    #[test]
    fn test_extract_measure_data_with_half_note_metronome() {
        // Arrange
        let score = create_musicxml_dom_with_half_note_metronome();

        // Act
        let actual = extract_measure_data(&score);

        // Assert
        let expected = vec![MeasureData {
            note_count: 0,
            tempo_bpm: 120.0,
            time_signature: TimeSignature::new(4, 4),
            pitches: HashSet::new(),
        }];
        assert_eq!(actual, expected);
    }

    fn create_musicxml_dom_with_half_note_metronome() -> ScorePartwise {
        let measures = format!(
            r#"<measure number="1">
    {}
</measure>"#,
            create_metronome("half", 60)
        );
        create_test_score(&measures)
    }

    #[test]
    fn test_extract_measure_data_with_dotted_half_note_metronome() {
        // Arrange
        let score = create_musicxml_dom_with_dotted_half_note_metronome();

        // Act
        let actual = extract_measure_data(&score);

        // Assert
        let expected = vec![MeasureData {
            note_count: 0,
            tempo_bpm: 60.0 * 3.0,
            time_signature: TimeSignature::new(3, 4),
            pitches: HashSet::new(),
        }];
        assert_eq!(actual, expected);
    }

    fn create_musicxml_dom_with_dotted_half_note_metronome() -> ScorePartwise {
        let measures = format!(
            r#"<measure number="1">
    {time_sig}
    {metronome}
</measure>"#,
            time_sig = create_time_signature(3, 4),
            metronome = create_dotted_metronome("half", 60)
        );
        create_test_score(&measures)
    }

    fn create_test_score(measures: &str) -> ScorePartwise {
        let xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE score-partwise PUBLIC "-//Recordare//DTD MusicXML 4.0 Partwise//EN" "http://www.musicxml.org/dtds/partwise.dtd">
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Test</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    {measures}
  </part>
</score-partwise>"#
        );
        parse_musicxml_to_dom(&xml)
    }

    fn create_note(step: &str, octave: u8) -> String {
        format!(
            r#"<note>
        <pitch>
          <step>{step}</step>
          <octave>{octave}</octave>
        </pitch>
        <duration>1</duration>
        <type>quarter</type>
      </note>"#
        )
    }

    fn create_rest() -> String {
        r#"<note>
        <rest/>
        <duration>1</duration>
        <type>quarter</type>
      </note>"#
            .to_string()
    }

    fn create_metronome(beat_unit: &str, per_minute: u32) -> String {
        format!(
            r#"<direction placement="above">
        <direction-type>
          <metronome>
            <beat-unit>{beat_unit}</beat-unit>
            <per-minute>{per_minute}</per-minute>
          </metronome>
        </direction-type>
      </direction>"#
        )
    }

    fn create_time_signature(beats: u32, beat_type: u32) -> String {
        format!(
            r#"<attributes>
        <time>
          <beats>{beats}</beats>
          <beat-type>{beat_type}</beat-type>
        </time>
      </attributes>"#
        )
    }

    fn create_dotted_metronome(beat_unit: &str, per_minute: u32) -> String {
        format!(
            r#"<direction placement="above">
    <direction-type>
        <metronome>
            <beat-unit>{beat_unit}</beat-unit>
            <beat-unit-dot/>
            <per-minute>{per_minute}</per-minute>
        </metronome>
    </direction-type>
</direction>"#
        )
    }

    fn parse_musicxml_to_dom(xml: &str) -> ScorePartwise {
        musicxml::read_score_data_partwise(xml.as_bytes().to_vec())
            .expect("Failed to parse test XML")
    }
}
