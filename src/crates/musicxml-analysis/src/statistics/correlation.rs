pub fn calculate_pearson_correlation(x_values: &[f64], y_values: &[f64]) -> f64 {
    if x_values.len() != y_values.len() || x_values.len() < 2 {
        return 0.0;
    }

    let n = x_values.len() as f64;
    let sum_x: f64 = x_values.iter().sum();
    let sum_y: f64 = y_values.iter().sum();
    let sum_xy: f64 = x_values
        .iter()
        .zip(y_values.iter())
        .map(|(x, y)| x * y)
        .sum();
    let sum_x_sq: f64 = x_values.iter().map(|x| x * x).sum();
    let sum_y_sq: f64 = y_values.iter().map(|y| y * y).sum();

    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = ((n * sum_x_sq - sum_x * sum_x) * (n * sum_y_sq - sum_y * sum_y)).sqrt();

    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::assert_float_absolute_eq;

    #[test]
    fn test_perfect_positive_correlation() {
        // Arrange
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

        // Act
        let correlation = calculate_pearson_correlation(&x, &y);

        // Assert
        assert_float_absolute_eq!(correlation, 1.0, 0.001);
    }

    #[test]
    fn test_perfect_negative_correlation() {
        // Arrange
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![10.0, 8.0, 6.0, 4.0, 2.0];

        // Act
        let correlation = calculate_pearson_correlation(&x, &y);

        // Assert
        assert_float_absolute_eq!(correlation, -1.0, 0.001);
    }

    #[test]
    fn test_no_correlation() {
        // Arrange
        let x = vec![1.0, 2.0, 3.0, 4.0];
        let y = vec![1.0, 4.0, 2.0, 3.0];

        // Act
        let correlation = calculate_pearson_correlation(&x, &y);

        // Assert
        assert_float_absolute_eq!(correlation, 0.0, 0.5);
    }

    #[test]
    fn test_empty_data() {
        // Arrange
        let x = vec![];
        let y = vec![];

        // Act
        let correlation = calculate_pearson_correlation(&x, &y);

        // Assert
        assert_float_absolute_eq!(correlation, 0.0);
    }

    #[test]
    fn test_mismatched_lengths() {
        // Arrange
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![1.0, 2.0];

        // Act
        let correlation = calculate_pearson_correlation(&x, &y);

        // Assert
        assert_float_absolute_eq!(correlation, 0.0);
    }

    #[test]
    fn test_single_value() {
        // Arrange
        let x = vec![5.0];
        let y = vec![10.0];

        // Act
        let correlation = calculate_pearson_correlation(&x, &y);

        // Assert
        assert_float_absolute_eq!(correlation, 0.0);
    }

    #[test]
    fn test_constant_values() {
        // Arrange
        let x = vec![3.0, 3.0, 3.0, 3.0];
        let y = vec![7.0, 7.0, 7.0, 7.0];

        // Act
        let correlation = calculate_pearson_correlation(&x, &y);

        // Assert
        assert_float_absolute_eq!(correlation, 0.0);
    }
}
