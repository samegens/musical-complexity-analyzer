use super::TimeSignature;

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
