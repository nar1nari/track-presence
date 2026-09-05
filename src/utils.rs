pub fn normalize_string(s: &str) -> String {
    s.to_lowercase().replace(' ', "")
}

pub fn error_chain(err: &(dyn std::error::Error + 'static)) -> String {
    let mut msg = err.to_string();
    let mut source = err.source();
    while let Some(e) = source {
        msg.push_str(": ");
        msg.push_str(&e.to_string());
        source = e.source();
    }
    msg
}
