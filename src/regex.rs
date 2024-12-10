use format as f;
use regex::Regex;
use std::sync::LazyLock;

// Regular expression pattern for matching numbers, including:
// - Optional sign (+ or -)
// - Integer or decimal numbers
// - Scientific notation (e.g., 1.23e-4)
// - Special values (inf, NaN)
const NUMERIC_VALUE: &'static str = r"(?:-|\+)?(?:\d*\.?\d+(?:e(?:-|\+)?\d+)?|inf|NaN)";

// Static regex patterns initialized lazily using OnceLock
pub static PARENTHESIS: LazyLock<Regex> =
    // Matches any expression inside parentheses that doesn't contain nested parentheses
    LazyLock::new(|| Regex::new(&f!(r"\s*\(([^()]+)\)\s*")).unwrap());

pub static EXPONENTIAL: LazyLock<Regex> =
    // Initializes regex for matching exponential operations: number ^ number
    LazyLock::new(|| {
        Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\^\s*({NUMERIC_VALUE})\s*")).unwrap()
    });

pub static DIVISION: LazyLock<Regex> =
    // Initializes regex for matching division operations: number / number
    LazyLock::new(|| {
        Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\/\s*({NUMERIC_VALUE})\s*")).unwrap()
    });

pub static MULTIPLICATION: LazyLock<Regex> =
    // Initializes regex for matching multiplication operations: number * number
    LazyLock::new(|| {
        Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\*\s*({NUMERIC_VALUE})\s*")).unwrap()
    });

pub static SUBTRACTION: LazyLock<Regex> =
    // Initializes regex for matching subtraction operations: number - number
    LazyLock::new(|| {
        Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*-\s*({NUMERIC_VALUE})\s*")).unwrap()
    });

pub static ADDITION: LazyLock<Regex> =
    // Initializes regex for matching addition operations: number + number
    LazyLock::new(|| {
        Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\+\s*({NUMERIC_VALUE})\s*")).unwrap()
    });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_values() {
        let valid = vec![
            "123", "-123", "+123", "0.123", "-0.123", "1e5", "-1.23e-4", "+1.23e+4", "inf", "-inf",
            "+inf", "NaN",
        ];

        for num in valid {
            assert!(Regex::new(&format!(r"^{NUMERIC_VALUE}$"))
                .unwrap()
                .is_match(num));
        }
    }

    #[test]
    fn test_parenthesis() {
        let cases = vec![
            // basic case
            ("(1 + 2)", "1 + 2"),
            // with prefix
            ("prefix (1.5*2.5)", "1.5*2.5"),
            // nested parenthesis
            ("(1 + (2 * 3))", "2 * 3"),
        ];

        for (expr, expected) in cases {
            let captures = PARENTHESIS.captures(expr).unwrap();

            assert_eq!(expected, captures.get(1).unwrap().as_str())
        }
    }

    #[test]
    fn test_exponential() {
        let valid_cases = vec![
            //basic case
            ("2^3", "2", "3"),
            // ignores whitespace
            (" 1.5 ^ 2.5 ", "1.5", "2.5"),
            // picks first match
            ("-2 ^ 3 ^ 7", "-2", "3"),
            // scientific notation
            ("2.5e-1 ^ 3.2e+2", "2.5e-1", "3.2e+2"),
        ];

        for (expr, expected1, expected2) in valid_cases {
            let captures = EXPONENTIAL.captures(expr).unwrap();
            assert_eq!(captures.get(1).unwrap().as_str(), expected1);
            assert_eq!(captures.get(2).unwrap().as_str(), expected2);
        }
    }

    #[test]
    fn test_division() {
        let valid_cases = vec![
            // basic case
            ("4/2", "4", "2"),
            // with decimals and whitespace
            (" 1.5 / 0.5 ", "1.5", "0.5"),
            // with signed numbers
            ("-4 / +2", "-4", "+2"),
            // picks first match
            ("-2 / 3 / 7", "-2", "3"),
            // with scientific notation
            ("1.2e-3 / 3.4e+5", "1.2e-3", "3.4e+5"),
        ];

        for (expr, expected1, expected2) in valid_cases {
            let captures = DIVISION.captures(expr).unwrap();
            assert_eq!(captures.get(1).unwrap().as_str(), expected1);
            assert_eq!(captures.get(2).unwrap().as_str(), expected2);
        }
    }

    #[test]
    fn test_multiplication() {
        let valid_cases = vec![
            // basic case
            ("2*3", "2", "3"),
            // with decimals and whitespace
            (" 1.5 * 2.5 ", "1.5", "2.5"),
            // with signed numbers
            ("-2 * +3", "-2", "+3"),
            // picks first match
            ("-2 * 3 * 7", "-2", "3"),
            // with scientific notation
            ("1.2e-3 * 3.4e+5", "1.2e-3", "3.4e+5"),
        ];

        for (expr, expected1, expected2) in valid_cases {
            let captures = MULTIPLICATION.captures(expr).unwrap();
            assert_eq!(captures.get(1).unwrap().as_str(), expected1);
            assert_eq!(captures.get(2).unwrap().as_str(), expected2);
        }
    }

    #[test]
    fn test_subtraction() {
        let valid_cases = vec![
            // basic case
            ("5-3", "5", "3"),
            // with decimals and whitespace
            (" 1.5 - 2.5 ", "1.5", "2.5"),
            // with signed numbers
            ("-2 - +3", "-2", "+3"),
            // picks first match
            ("-2 - 3 - 7", "-2", "3"),
            // with scientific notation
            ("1.2e-3 - 3.4e+5", "1.2e-3", "3.4e+5"),
        ];

        for (expr, expected1, expected2) in valid_cases {
            let captures = SUBTRACTION.captures(expr).unwrap();
            assert_eq!(captures.get(1).unwrap().as_str(), expected1);
            assert_eq!(captures.get(2).unwrap().as_str(), expected2);
        }
    }

    #[test]
    fn test_addition() {
        let valid_cases = vec![
            // basic case
            ("2+3", "2", "3"),
            // with decimals and whitespace
            (" 1.5 + 2.5 ", "1.5", "2.5"),
            // with signed numbers
            ("-2 + +3", "-2", "+3"),
            // picks first match
            ("-2 + 3 + 7", "-2", "3"),
            // with scientific notation
            ("1.2e-3 + 3.4e+5", "1.2e-3", "3.4e+5"),
        ];

        for (expr, expected1, expected2) in valid_cases {
            let captures = ADDITION.captures(expr).unwrap();
            assert_eq!(captures.get(1).unwrap().as_str(), expected1);
            assert_eq!(captures.get(2).unwrap().as_str(), expected2);
        }
    }
}
