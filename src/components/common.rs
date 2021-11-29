use regex::Regex;

pub fn regex_filter(
    regex_string: &str,
    match_string: &str,
    case_sensitive: bool,
) -> bool {
    let mut regex_string = regex_string.to_string();

    if case_sensitive {
        regex_string.push_str("/i");
    }

    if let Ok(regex) = Regex::new(&regex_string) {
        return regex.captures_iter(match_string).next().is_some();
    }

    false
}
