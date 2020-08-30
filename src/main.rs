use std::fs;

use yaml_rust::yaml::Array;

use crate::args_mapper::{map_args, SubCommand};
use crate::config_mapper::map_config;
use crate::parser::{hosts_map_to_string, parse_hosts, read_hosts};

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
    let mut hosts = parse_hosts(&hosts).expect("Invalid config file");
    let config = &map_config(&args.config)[0];
    //match sub commands
    match args.sub_cmd {
        SubCommand::Switch(s) => {
            let profile = &config[&s.profile[..]];
            if profile.is_badvalue() {
                panic!("Profile {} not found", s.profile);
            }
            println!("Switching to profile {}", s.profile);
            let profile = profile.as_vec().unwrap();
            let mut str_vec = Vec::with_capacity(profile.len() + 1);
            str_vec.push("localhost");
            for s in profile {
                str_vec.push(s.as_str().unwrap());
            }
            hosts.insert("127.0.0.1", str_vec);
            fs::write(&args.file, hosts_map_to_string(&hosts)).expect("Error writing to hosts file");
        }
    }
}