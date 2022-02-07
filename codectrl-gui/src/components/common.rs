use regex::RegexBuilder;

pub fn regex_filter(
    regex_string: &str,
    match_string: &str,
    case_sensitive: bool,
) -> bool {
    if let Ok(regex) = RegexBuilder::new(regex_string)
        .case_insensitive(!case_sensitive)
        .build()
    {
        return regex.captures_iter(match_string).next().is_some();
    }

    false
}
