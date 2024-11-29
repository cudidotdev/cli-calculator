use regex::Regex;

pub fn extract_regex_captures<'a, const N: usize>(
    input: &'a str,
    pattern: &Regex,
) -> Option<(usize, usize, [&'a str; N])> {
    // Attempt to find captures within the input string.
    let captures = pattern.captures(input)?;

    // Extract the start and end positions of the entire match.
    let (start, end) = if let Some(full_match) = captures.get(0) {
        (full_match.start(), full_match.end())
    } else {
        return None;
    };

    // Extract captured substrings and convert them into an array of size N.
    let (_, extracted_captures) = captures.extract::<N>();

    // Return the positions of the match and the array of captured substrings.
    Some((start, end, extracted_captures))
}
