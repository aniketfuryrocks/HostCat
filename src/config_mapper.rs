use std::fs;

use yaml_rust::{Yaml, YamlLoader};

pub fn map_config(path: &str) -> Vec<Yaml> {
    YamlLoader::load_from_str(
        &fs::read_to_string(path)
            .expect(&format!("Error reading config {}", path)))
        .unwrap()
}