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

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_basic_extraction() {
        let pattern = Regex::new(r"(\w+):(\d+)").unwrap();
        let input = "test:123";

        let result = extract_regex_captures::<2>(input, &pattern);
        assert!(result.is_some());

        let (start, end, captures) = result.unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 8);
        assert_eq!(captures, ["test", "123"]);
    }

    #[test]
    fn test_multiple_matches_takes_first() {
        let pattern = Regex::new(r"(\w+):(\d+)").unwrap();
        let input = "prefix test:123 other:456 suffix";

        let result = extract_regex_captures::<2>(input, &pattern);
        assert!(result.is_some());

        let (start, end, captures) = result.unwrap();
        assert_eq!(start, 7);
        assert_eq!(end, 15);
        assert_eq!(captures, ["test", "123"]);
    }

    #[test]
    fn test_no_match() {
        let pattern = Regex::new(r"(\w+):(\d+)").unwrap();
        let input = "no match here";

        let result = extract_regex_captures::<2>(input, &pattern);
        assert_eq!(result, None);
    }

    #[test]
    #[should_panic]
    fn test_wrong_capture_group_count() {
        let pattern = Regex::new(r"(\w+):(\d+)").unwrap();
        let input = "test:123";

        // This should panic because we're requesting 3 captures but pattern only has 2
        let _ = extract_regex_captures::<3>(input, &pattern);
    }

    #[test]
    #[ignore]
    fn test_large_input_performance() {
        let pattern = Regex::new(r"(\w+):(\d+)").unwrap();
        let large_input = "prefix ".repeat(1000000) + "test:123";

        let result = extract_regex_captures::<2>(&large_input, &pattern);
        assert!(result.is_some());

        let (_, _, captures) = result.unwrap();
        assert_eq!(captures, ["test", "123"]);
    }

    #[test]
    fn test_empty_input() {
        let pattern = Regex::new(r"(\w+):(\d+)").unwrap();
        let input = "";

        let result = extract_regex_captures::<2>(input, &pattern);
        assert!(result.is_none());
    }
}
