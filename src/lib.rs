mod calculator;
mod evaluator;
mod regex;
mod utils;

pub use crate::calculator::Calculator;
pub use crate::evaluator::{EvaluationError, Evaluator, EvaluatorFn};
use crate::regex::{ADDITION, DIVISION, EXPONENTIAL, MULTIPLICATION, PARENTHESIS, SUBTRACTION};

use ::regex::Regex;

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

// Special evaluator for handling parenthesized expressions
pub struct Parenthesis;

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
        &PARENTHESIS
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
pub struct Exponential;

impl Evaluator<2> for Exponential {
    fn regex() -> &'static Regex {
        &EXPONENTIAL
    }

    fn operator(operands: [f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0].powf(operands[1]))
    }
}

// Division implementation
pub struct Division;

impl Evaluator<2> for Division {
    fn regex() -> &'static Regex {
        &DIVISION
    }

    fn operator(operands: [f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0] / operands[1])
    }
}

// Multiplication implementation
pub struct Multiplication;

impl Evaluator<2> for Multiplication {
    fn regex() -> &'static Regex {
        &MULTIPLICATION
    }

    fn operator(operands: [f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0] * operands[1])
    }
}

// Similar structure for Subtraction
pub struct Subtraction;

impl Evaluator<2> for Subtraction {
    fn regex() -> &'static Regex {
        &SUBTRACTION
    }

    fn operator(operands: [f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0] - operands[1])
    }
}

// Implementation for each arithmetic operation
pub struct Addition;

impl Evaluator<2> for Addition {
    // Generic parameter 2 indicates binary operation
    fn regex() -> &'static Regex {
        &ADDITION
    }

    // Performs the actual addition operation
    fn operator(operands: [f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0] + operands[1])
    }
}
