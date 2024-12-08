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
    // Initializes regex for matching addition operations: number ^ number
    LazyLock::new(|| {
        Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\^\s*({NUMERIC_VALUE})\s*")).unwrap()
    });

pub static DIVISION: LazyLock<Regex> =
    // Initializes regex for matching addition operations: number / number
    LazyLock::new(|| {
        Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\/\s*({NUMERIC_VALUE})\s*")).unwrap()
    });

pub static MULTIPLICATION: LazyLock<Regex> =
    // Initializes regex for matching addition operations: number * number
    LazyLock::new(|| {
        Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\*\s*({NUMERIC_VALUE})\s*")).unwrap()
    });

pub static SUBTRACTION: LazyLock<Regex> =
    // Initializes regex for matching addition operations: number - number
    LazyLock::new(|| {
        Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*-\s*({NUMERIC_VALUE})\s*")).unwrap()
    });

pub static ADDITION: LazyLock<Regex> =
    // Initializes regex for matching addition operations: number + number
    LazyLock::new(|| {
        Regex::new(&f!(r"\s*({NUMERIC_VALUE})\s*\+\s*({NUMERIC_VALUE})\s*")).unwrap()
    });
