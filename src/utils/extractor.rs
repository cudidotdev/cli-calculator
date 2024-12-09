use regex::Regex;

// Extracts regex captures from input string
// Generic parameter N specifies number of capture groups to extract
pub fn extract_regex_captures<'a, const N: usize>(
    input: &'a str,
    pattern: &Regex,
) -> Option<(usize, usize, [&'a str; N])> {
    // Try to find matches in the input string
    let captures = pattern.captures(input)?;

    // Get the start and end positions of the entire match
    let (start, end) = if let Some(full_match) = captures.get(0) {
        (full_match.start(), full_match.end())
    } else {
        return None;
    };

    // Extract exactly N capture groups into an array
    let (_, extracted_captures) = captures.extract::<N>();

    // Return tuple of:
    // - start position of match
    // - end position of match
    // - array of captured strings
    Some((start, end, extracted_captures))
}
