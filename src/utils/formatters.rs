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
