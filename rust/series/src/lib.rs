pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return Vec::new();
    }

    (0..=digits.len() - len)
        .map(|start| (&digits[start..start + len]).to_string())
        .collect()
}
