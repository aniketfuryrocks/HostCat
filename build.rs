use std::{env, fs};

fn main() {
    let config_file = format!("{}/{}", env::home_dir().unwrap().to_str().unwrap(), ".hostcat");
    match fs::write(&config_file, "default localhost") {
        Err(e) => {
            println!("cargo:warning=Error creating default config file [{}] {}", &config_file, e.to_string());
            println!("cargo:warning=Manually create /etc/hostcat or specify one using -c. [hostcat help] for more info")
        }
        _ => {}
    }
}