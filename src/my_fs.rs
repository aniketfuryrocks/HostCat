use std::env;

pub fn validate_tilde(mut path: String) -> String {
    if path.starts_with("~") {
        path.replace_range(0..1, env::home_dir().unwrap().to_str().unwrap());
    }
    path
}