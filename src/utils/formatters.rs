// Formats a floating point number and removes unnecessary decimal places
pub fn format_and_trim(input: f64) -> String {
    // Steps:
    // 1. Format number to 10 decimal places
    // 2. Remove trailing zeros (e.g., 1.2300 -> 1.23)
    // 3. Remove decimal point if no decimals remain (e.g., 1. -> 1)
    let formatted_input = format!("{input:.10}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned();

    formatted_input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        assert_eq!(format_and_trim(1.23000), "1.23");
        assert_eq!(format_and_trim(5.00000), "5");
    }

    #[test]
    fn test_zero() {
        assert_eq!(format_and_trim(000.00000), "0");
        assert_eq!(format_and_trim(0.0), "0");
    }

    #[test]
    fn test_long_decimals() {
        assert_eq!(format_and_trim(0.300000000000004), "0.3");

        assert_eq!(format_and_trim(100.123456789012), "100.123456789");
    }

    #[test]
    fn test_negative_values() {
        assert_eq!(format_and_trim(-3.4000), "-3.4");
        assert_eq!(format_and_trim(-0.00001), "-0.00001");
    }

    #[test]
    fn test_no_fractional_part() {
        assert_eq!(format_and_trim(42.0), "42");
        assert_eq!(format_and_trim(-42.0), "-42");
    }
}
