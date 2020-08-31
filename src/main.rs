use std::fs;

use crate::args_mapper::{map_args, SubCommand};
use crate::parser::{hosts_map_to_string, parse_hosts, read_hosts};

mod parser;
mod privileges;
mod args_mapper;

fn main() {
    //map args
    let args = map_args();
    //optional check for root privileges
    if !args.root_unchecked {
        privileges::check_privileges();
    }
    //read host file-
    let hosts = read_hosts(&args.file);
    let mut hosts = parse_hosts(&hosts).expect("Invalid host file");
    let config = fs::read_to_string(&args.config).expect(&format!("Error reading config from {}", &args.config));
    let mut config = parse_hosts(&config).expect("Invalid Config file");
    //match sub commands
    match args.sub_cmd {
        SubCommand::Switch(s) => {
            match config.get_mut(&s.profile[..]) {
                None => panic!("Profile {} does not exist", s.profile),
                Some(profile) => {
                    println!("Switching to profile {}", s.profile);
                    if !profile.contains(&"localhost") {
                        profile.push("localhost");
                    }
                    hosts.insert("127.0.0.1", profile.to_vec());
                    fs::write(&args.file, hosts_map_to_string(&hosts)).expect("Error writing to hosts file");
                }
            }
        }
        SubCommand::Set(s) => {
            //check if value was provided
            /*if s.value.is_empty() {
                panic!("Value not specified, use -v option to specify one");
            }
            //overwrite profile
            let arr: &mut Vec<Yaml> = config[&s.profile[..]].as_vec().unwrap().as_mut();
            arr.push(YamlLoader::load_from_str(&format!("[{}]", s.value)).unwrap()[0].to_owned());
            //inform
            println!("Writing to profile {}", s.profile);
            //write to file
            println!("{:?}", config);*/
        }
    }
}