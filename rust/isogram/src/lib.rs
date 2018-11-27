use std::collections::HashSet;

fn duplicate_chars<'a>(candidate: &'a str) -> impl Iterator<Item = char> + 'a {
    let mut found: HashSet<String> = HashSet::new();
    candidate
        .chars()
        .filter(move |c| !found.insert(c.to_lowercase().to_string()))
}

/// Check if candidate is an isogram
/// Works for unicode strings
pub fn check(candidate: &str) -> bool {
    duplicate_chars(candidate).all(|c| !c.is_alphanumeric())
}
