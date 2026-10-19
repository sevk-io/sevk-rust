macro_rules! regex {
    ($pattern:expr) => {{
        static PATTERN: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        PATTERN.get_or_init(|| regex::Regex::new($pattern).unwrap())
    }};
}
