pub fn calculate_note_density(total_notes: u32, duration_seconds: f64) -> f64 {
    total_notes as f64 / duration_seconds
}

#[cfg(test)]
mod tests {
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
        assert_eq!(actual, expected);
    }
}
