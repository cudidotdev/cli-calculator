#![allow(dead_code)]

mod utils;

mod evaluator;

mod calculator;

use std::sync::OnceLock;

use regex::Regex;

use crate::evaluator::{EvaluationError, Evaluator, EvaluatorFn};

pub use crate::calculator::Calculator;

use format as f;

pub struct Calculate;

impl Calculator for Calculate {
    fn evaluators() -> Vec<EvaluatorFn> {
        vec![
            Parenthesis::evalutate,
            Exponential::evalutate,
            Division::evalutate,
            Multiplication::evalutate,
            Subtraction::evalutate,
            Addition::evalutate,
        ]
    }
}

const NUMERIC_VALUE: &'static str = r"(?:-|\+)?(?:\d*\.?\d+(?:e(?:-|\+)?\d+)?|inf|NaN)";

static PARENTHESIS: OnceLock<Regex> = OnceLock::new();

static EXPONENTIAL: OnceLock<Regex> = OnceLock::new();

static MULTIPLICATION: OnceLock<Regex> = OnceLock::new();

static DIVISION: OnceLock<Regex> = OnceLock::new();

static ADDITION: OnceLock<Regex> = OnceLock::new();

static SUBTRACTION: OnceLock<Regex> = OnceLock::new();

struct Addition;

impl Evaluator<2> for Addition {
    fn regex() -> &'static Regex {
        ADDITION.get_or_init(|| {
            Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\+\s*({NUMERIC_VALUE})\s*")).unwrap()
        })
    }

    fn operator(operands: &[f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0] + operands[1])
    }
}

struct Subtraction;

impl Evaluator<2> for Subtraction {
    fn regex() -> &'static Regex {
        SUBTRACTION.get_or_init(|| {
            Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*-\s*({NUMERIC_VALUE})\s*")).unwrap()
        })
    }

    fn operator(operands: &[f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0] - operands[1])
    }
}

struct Multiplication;

impl Evaluator<2> for Multiplication {
    fn regex() -> &'static Regex {
        MULTIPLICATION.get_or_init(|| {
            Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\*\s*({NUMERIC_VALUE})\s*")).unwrap()
        })
    }

    fn operator(operands: &[f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0] * operands[1])
    }
}

struct Division;

impl Evaluator<2> for Division {
    fn regex() -> &'static Regex {
        DIVISION.get_or_init(|| {
            Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\/\s*({NUMERIC_VALUE})\s*")).unwrap()
        })
    }

    fn operator(operands: &[f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0] / operands[1])
    }
}

struct Exponential;

impl Evaluator<2> for Exponential {
    fn regex() -> &'static Regex {
        EXPONENTIAL.get_or_init(|| {
            Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\^\s*({NUMERIC_VALUE})\s*")).unwrap()
        })
    }

    fn operator(operands: &[f64; 2]) -> Result<f64, EvaluationError> {
        Ok(operands[0].powf(operands[1]))
    }
}

struct Parenthesis;

impl Calculator for Parenthesis {
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
    fn regex() -> &'static Regex {
        PARENTHESIS.get_or_init(|| Regex::new(&f!(r"\s*\(([^()]+)\)\s*")).unwrap())
    }

    fn parser(input: &str) -> Result<f64, EvaluationError> {
        Self::calculate(input.into())
    }

    fn operator(operands: &[f64; 1]) -> Result<f64, EvaluationError> {
        Ok(operands[0])
    }
}
