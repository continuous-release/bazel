pub const VERSION: &str = include_str!("../../version.txt");

pub fn version() -> &'static str {
    VERSION.trim()
}

pub fn message() -> String {
    format!("Hello World! (v{})", version())
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_is_not_empty() {
        assert!(!crate::version().is_empty());
    }
}
