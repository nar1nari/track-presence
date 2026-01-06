pub fn normalize_string(s: &str) -> String {
    s.to_lowercase().replace(' ', "")
}
