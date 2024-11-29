use crate::utils::{
    extractor::extract_regex_captures,
    formatters::{add_explicit_sign, format_and_trim},
    parser::parse_value,
};

use regex::Regex;

use thiserror::Error;

pub type EvaluatorFn = fn(input: String) -> Result<String, EvaluationError>;

#[derive(Error, Debug, PartialEq)]
pub enum EvaluationError {
    #[error("Error parsing token: {0}")]
    ParseError(String),

    #[error("Unknown error")]
    UnknownError,
}

pub trait Evaluator<const N: usize> {
    fn regex() -> &'static Regex;

    fn parser(input: &str) -> Result<f64, EvaluationError> {
        parse_value(input)
    }

    fn extractor(input: &str) -> Option<(usize, usize, [&str; N])> {
        extract_regex_captures::<N>(input, Self::regex())
    }

    fn operator(operands: &[f64; N]) -> Result<f64, EvaluationError>;

    fn formatter(input: f64) -> String {
        add_explicit_sign(format_and_trim(input))
    }

    fn evalutate(input: String) -> Result<String, EvaluationError> {
        // Extract the match and captures using the provided extraction function
        let Some((start, end, operands)) = Self::extractor(&input) else {
            return Ok(input.to_owned()); // No division found, return the input unchanged
        };

        // parse vector of extracted operands into boxed floats
        let boxed_operands: Box<[f64; N]> = operands
            .into_iter()
            .map(|e| Self::parser(e))
            .collect::<Result<Vec<_>, _>>()
            // unwrapped since this won't error
            // the operands are of length N and
            // the box is of length N
            .map(|vec| vec.try_into().unwrap())?;

        // Parse the extracted numbers and perform the division
        let result = Self::operator(&boxed_operands.map(|s| s))?;

        let formatted_result = Self::formatter(result);

        // Replace the matched part in the input with the formatted result
        let mut modified_input = input.to_owned();
        modified_input.replace_range(start..end, &formatted_result);

        Self::evalutate(modified_input)
    }
}
