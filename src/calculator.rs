#![allow(dead_code)]

use crate::{
    evaluator::{EvaluationError, EvaluatorFn},
    utils::parser::parse_value,
};

// Main trait that defines calculator behavior
pub trait Calculator {
    // Returns a vector of evaluation functions in order of precedence
    // Each implementation defines its own order of operations
    fn evaluators() -> Vec<EvaluatorFn>;

    // Converts string input to f64
    // Can be overridden by implementations if custom parsing is needed
    fn parser(input: String) -> Result<f64, EvaluationError> {
        parse_value(input)
    }

    // Main calculation function that processes a mathematical expression
    fn calculate(input: String) -> Result<f64, EvaluationError> {
        // Iteratively apply each evaluator function in order
        // try_fold processes the string through each evaluator, handling errors
        let result = Self::evaluators().iter().try_fold(input, |acc, f| f(acc))?;

        // Convert final string result to f64
        Self::parser(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockCalculator;

    impl Calculator for MockCalculator {
        fn evaluators() -> Vec<EvaluatorFn> {
            vec![mock_fn_a, mock_fn_b, mock_fn_c]
        }

        fn parser(input: String) -> Result<f64, EvaluationError> {
            if input.ends_with("abc") {
                Ok(1.0)
            } else {
                Err(EvaluationError::UnknownError)
            }
        }
    }

    fn mock_fn_a(input: String) -> Result<String, EvaluationError> {
        Ok(input + "a")
    }

    fn mock_fn_b(input: String) -> Result<String, EvaluationError> {
        Ok(input + "b")
    }

    fn mock_fn_c(input: String) -> Result<String, EvaluationError> {
        Ok(input + "c")
    }

    #[test]
    fn test_calculate_method() -> Result<(), EvaluationError> {
        assert_eq!(MockCalculator::calculate("somestring".into())?, 1.0);

        Ok(())
    }
}
