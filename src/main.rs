use yaml_rust::yaml::Array;

use crate::args_mapper::{map_args, SubCommand};
use crate::config_mapper::map_config;
use crate::parser::{parse_hosts, read_hosts};

mod parser;
mod privileges;
mod args_mapper;
mod config_mapper;

fn main() {
    //map args
    let args = map_args();
    //optional check for root privileges
    if !args.root_unchecked {
        privileges::check_privileges();
    }
    //read host file
    let hosts = read_hosts(&args.file);
    let hosts = parse_hosts(&hosts).expect("Invalid config file");
    let config = &map_config(&args.config)[0];
    //match sub commands
    match args.sub_cmd {
        SubCommand::Switch(s) => {
            let profile = &config[&s.profile[..]];
            if profile.is_badvalue() {
                println!("Profile {} not found", s.profile);
            } else {
                println!("Switching to profile {}", s.profile);
                let profile = profile.as_vec().unwrap();
                println!("Hots {:?}", profile);
            }
        }
    }
    println!("{:?}", hosts);
}