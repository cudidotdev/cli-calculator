mod calculator;
mod evaluator;
mod utils;

pub use crate::calculator::Calculator;
pub use crate::evaluator::{EvaluationError, Evaluator, EvaluatorFn};
use regex::Regex;
use std::sync::OnceLock; // Thread-safe lazy initialization for static values

use format as f; // Alias for format macro

// Main calculator implementation
pub struct Calculate;

impl Calculator for Calculate {
    // Defines the order of operations (PEMDAS)
    // Note: Division before multiplication and subtraction before addition works
    // independently - the reverse order would require dependency handling
    fn evaluators() -> Vec<EvaluatorFn> {
        vec![
            Parenthesis::evalutate, // Highest priority
            Exponential::evalutate, // Then exponents
            Division::evalutate,    // Then multiplication/division
            Multiplication::evalutate,
            Subtraction::evalutate, // Then addition/subtraction
            Addition::evalutate,
        ]
    }
}

// Regular expression pattern for matching numbers, including:
// - Optional sign (+ or -)
// - Integer or decimal numbers
// - Scientific notation (e.g., 1.23e-4)
// - Special values (inf, NaN)
const NUMERIC_VALUE: &'static str = r"(?:-|\+)?(?:\d*\.?\d+(?:e(?:-|\+)?\d+)?|inf|NaN)";

// Static regex patterns initialized lazily using OnceLock
static PARENTHESIS: OnceLock<Regex> = OnceLock::new();
static EXPONENTIAL: OnceLock<Regex> = OnceLock::new();
static MULTIPLICATION: OnceLock<Regex> = OnceLock::new();
static DIVISION: OnceLock<Regex> = OnceLock::new();
static ADDITION: OnceLock<Regex> = OnceLock::new();
static SUBTRACTION: OnceLock<Regex> = OnceLock::new();

// Special evaluator for handling parenthesized expressions
struct Parenthesis;

impl Calculator for Parenthesis {
    // Defines the order of operations (PEMDAS)
    // Note: Division before multiplication and subtraction before addition works
    // independently - the reverse order would require dependency handling
    fn evaluators() -> Vec<EvaluatorFn> {
        vec![
            Exponential::evalutate,
            Division::evalutate,
            Multiplication::evalutate,
            Subtraction::evalutate,
            Addition::evalutate,
        ]
    }
}

impl Evaluator<1> for Parenthesis {
    // Takes 1 parameter (the expression inside parentheses)
    fn regex() -> &'static Regex {
        // Matches any expression inside parentheses that doesn't contain nested parentheses
        PARENTHESIS.get_or_init(|| Regex::new(&f!(r"\s*\(([^()]+)\)\s*")).unwrap())
    }

    // Parses the expression inside parentheses by recursively calculating it
    fn parser(input: String) -> Result<f64, EvaluationError> {
        Self::calculate(input)
    }

    // Simply returns the calculated value
    fn operator(operands: [f64; 1]) -> Result<f64, EvaluationError> {
        Ok(operands[0])
    }
}

// Exponential (power) implementation
struct Exponential;

impl Evaluator<2> for Exponential {
    fn regex() -> &'static Regex {
        EXPONENTIAL.get_or_init(|| {
            Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\^\s*({NUMERIC_VALUE})\s*")).unwrap()
        })
    }

    fn operator(operands: [f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0].powf(operands[1]))
    }
}

// Division implementation
struct Division;

impl Evaluator<2> for Division {
    fn regex() -> &'static Regex {
        DIVISION.get_or_init(|| {
            Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\/\s*({NUMERIC_VALUE})\s*")).unwrap()
        })
    }

    fn operator(operands: [f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0] / operands[1])
    }
}

// Multiplication implementation
struct Multiplication;

impl Evaluator<2> for Multiplication {
    fn regex() -> &'static Regex {
        MULTIPLICATION.get_or_init(|| {
            Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\*\s*({NUMERIC_VALUE})\s*")).unwrap()
        })
    }

    fn operator(operands: [f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0] * operands[1])
    }
}

// Similar structure for Subtraction
struct Subtraction;

impl Evaluator<2> for Subtraction {
    fn regex() -> &'static Regex {
        SUBTRACTION.get_or_init(|| {
            Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*-\s*({NUMERIC_VALUE})\s*")).unwrap()
        })
    }

    fn operator(operands: [f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0] - operands[1])
    }
}

// Implementation for each arithmetic operation
struct Addition;

impl Evaluator<2> for Addition {
    // Generic parameter 2 indicates binary operation
    fn regex() -> &'static Regex {
        // Initializes regex for matching addition operations: number + number
        ADDITION.get_or_init(|| {
            Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\+\s*({NUMERIC_VALUE})\s*")).unwrap()
        })
    }

    // Performs the actual addition operation
    fn operator(operands: [f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0] + operands[1])
    }
}
