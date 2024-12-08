use crate::utils::{
    extractor::extract_regex_captures, formatters::format_and_trim, parser::parse_value,
};
use regex::Regex;
use thiserror::Error;

// Type alias for evaluation functions
pub type EvaluatorFn = fn(String) -> Result<String, EvaluationError>;

// Custom error types for the calculator
#[derive(Error, Debug, PartialEq)]
pub enum EvaluationError {
    #[error("Error parsing token: {0}")]
    ParseError(String),

    #[error("Unknown error")]
    UnknownError,
}

// Main trait for implementing evaluators
// Generic parameter N represents number of operands (1 for unary, 2 for binary operations)
pub trait Evaluator<const N: usize> {
    // Returns the regex pattern for matching this operation
    fn regex() -> &'static Regex;

    // Converts string input to f64. Can be overridden for custom parsing
    fn parser(input: String) -> Result<f64, EvaluationError> {
        parse_value(input)
    }

    // Extracts matched operation and its operands from the input string
    fn extractor(input: &str) -> Option<(usize, usize, [&str; N])> {
        extract_regex_captures::<N>(input, Self::regex())
    }

    // Performs the actual mathematical operation
    fn operator(operands: [f64; N]) -> Result<f64, EvaluationError>;

    // Formats the result. Can be overridden for custom formatting
    fn formatter(input: f64) -> String {
        format_and_trim(input)
    }

    // Main evaluation function that processes the input string
    fn evalutate(input: String) -> Result<String, EvaluationError> {
        // Try to find a matching operation in the input
        let Some((start, end, captures)) = Self::extractor(&input) else {
            return Ok(input.to_owned()); // No match found, return unchanged
        };

        // Convert captured strings to numbers
        let mut operands = [0.0; N];
        for (i, capture) in captures.into_iter().enumerate() {
            operands[i] = Self::parser(capture.into())?
        }

        // Perform the operation
        let result = Self::operator(operands)?;

        // Format the result
        let formatted_result = Self::formatter(result);

        // Replace the matched operation with its result in the input string
        let mut modified_input = input.to_owned();
        modified_input.replace_range(start..end, &formatted_result);

        // Recursively evaluate remaining operations
        Self::evalutate(modified_input)
    }
}
