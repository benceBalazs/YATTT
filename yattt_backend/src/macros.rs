// macro definition to reduce repetitive code
macro_rules! lazy_env_var {
    ($name:ident, $env_var:expr, $default:expr) => {
        pub static $name: std::sync::LazyLock<String> =
            std::sync::LazyLock::new(|| match std::env::var($env_var) {
                Ok(value) if !value.is_empty() => value,
                _ => $default,
            });
    };
}