pub fn format_and_trim(input: f64) -> String {
    // Format the input to 10 decimal places and remove trailing zeros
    let formatted_input = format!("{input:.10}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned();

    formatted_input
}

pub fn add_explicit_sign(input: String) -> String {
    // Ensure it has an explicit sign
    if !input.starts_with('-') {
        return format!("+{input}");
    }

    input
}
