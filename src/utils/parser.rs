use crate::EvaluationError;

// Converts string representation of a number to f64
pub fn parse_value(input: String) -> Result<f64, EvaluationError> {
    // Attempt to parse string as f64
    // If parsing fails, wrap the original input string in a ParseError
    Ok(input
        .parse::<f64>()
        .map_err(|_| EvaluationError::ParseError(input))?)
}
