#[cfg(debug_assertions)]
pub fn format_error(error: Box<dyn std::error::Error>, error_message: &'static str) -> String {
    format!("{}, error: {}", error_message, error)
}

#[cfg(not(debug_assertions))]
pub fn format_error(_error: Box<dyn std::error::Error>, error_message: &'static str) -> String {
    error_message.to_string()
}
