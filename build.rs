use std::fs;

use crate::src::my_fs::validate_tilde;

mod src;

fn main() {
    let config_file = validate_tilde("~/.hostcat".to_string());
    match fs::write(&config_file, "default localhost") {
        Err(e) => {
            println!("cargo:warning=Error creating default config file [{}] {}", &config_file, e.to_string());
            println!("cargo:warning=Manually create /etc/hostcat or specify one using -c. [hostcat help] for more info")
        }
        _ => {}
    }
}