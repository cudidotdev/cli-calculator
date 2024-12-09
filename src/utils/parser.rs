use crate::EvaluationError;

// Converts string representation of a number to f64
pub fn parse_value(input: String) -> Result<f64, EvaluationError> {
    // Attempt to parse string as f64
    // If parsing fails, wrap the original input string in a ParseError
    input
        .parse::<f64>()
        .map_err(|_| EvaluationError::ParseError(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_regular_numbers() -> Result<(), EvaluationError> {
        // Integer values
        assert_eq!(parse_value("42".into())?, 42.0);
        assert_eq!(parse_value("-42".into())?, -42.0);
        assert_eq!(parse_value("0".into())?, 0.0);

        // Decimal values
        assert_eq!(parse_value("3.14".into())?, 3.14);
        assert_eq!(parse_value("-3.14".into())?, -3.14);
        assert_eq!(parse_value("0.0".into())?, 0.0);
        assert_eq!(parse_value(".5".into())?, 0.5);

        Ok(())
    }

    #[test]
    fn test_parse_scientific_notation() -> Result<(), EvaluationError> {
        // Positive exponents
        assert_eq!(parse_value("1e2".into())?, 100.0);
        assert_eq!(parse_value("1.5e2".into())?, 150.0);
        assert_eq!(parse_value("1E2".into())?, 100.0);

        // Negative exponents
        assert_eq!(parse_value("1e-2".into())?, 0.01);
        assert_eq!(parse_value("-1.5e-2".into())?, -0.015);

        // Zero exponents
        assert_eq!(parse_value("1.5e0".into())?, 1.5);

        Ok(())
    }

    #[test]
    fn test_parse_special_values() -> Result<(), EvaluationError> {
        // Infinity
        assert_eq!(parse_value("inf".into())?, f64::INFINITY);
        assert_eq!(parse_value("infinity".into())?, f64::INFINITY);

        // Negative infinity
        assert_eq!(parse_value("-inf".into())?, f64::NEG_INFINITY);
        assert_eq!(parse_value("-infinity".into())?, f64::NEG_INFINITY);

        // NaN
        assert!(parse_value("NaN".into())?.is_nan());
        assert!(parse_value("nan".into())?.is_nan());

        Ok(())
    }

    #[test]
    fn test_parse_invalid_input() {
        // Empty string
        assert!(matches!(
            parse_value("".to_string()),
            Err(EvaluationError::ParseError(_))
        ));

        // Invalid characters
        assert!(matches!(
            parse_value("abc".to_string()),
            Err(EvaluationError::ParseError(_))
        ));

        // Multiple decimal points
        assert!(matches!(
            parse_value("1.2.3".to_string()),
            Err(EvaluationError::ParseError(_))
        ));

        // Invalid scientific notation
        assert!(matches!(
            parse_value("1e".to_string()),
            Err(EvaluationError::ParseError(_))
        ));
        assert!(matches!(
            parse_value("1e-".to_string()),
            Err(EvaluationError::ParseError(_))
        ));

        // Spaces
        assert!(matches!(
            parse_value(" 42".to_string()),
            Err(EvaluationError::ParseError(_))
        ));
        assert!(matches!(
            parse_value("42 ".to_string()),
            Err(EvaluationError::ParseError(_))
        ));
    }
}
