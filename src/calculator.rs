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
