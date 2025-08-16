#[derive(Debug, PartialEq)]
pub struct DensityMetrics {
    pub average_notes_per_second: f64,
    pub peak_notes_per_second: f64,
    pub peak_measure: u32,
    pub total_note_count: u32,
}
