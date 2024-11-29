use crate::EvaluationError;

pub fn parse_value(input: &str) -> Result<f64, EvaluationError> {
    Ok(input
        .parse::<f64>()
        .map_err(|_| EvaluationError::ParseError(input.into()))?)
}
