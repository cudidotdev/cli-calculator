use crate::{
    evaluator::{EvaluationError, EvaluatorFn},
    utils::parser::parse_value,
};

pub trait Calculator {
    fn evaluators() -> Vec<EvaluatorFn>;

    fn parser(input: &str) -> Result<f64, EvaluationError> {
        parse_value(input)
    }

    fn calculate(input: String) -> Result<f64, EvaluationError> {
        let mut input = input;

        for evaluator in Self::evaluators() {
            input = evaluator(input)?
        }

        parse_value(&input)
    }
}
