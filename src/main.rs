use crate::args_mapper::{map_args, SubCommand};
use crate::parser::{parse_hosts, read_hosts};

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
    //read host file
    let hosts = read_hosts(&args.config);
    let hosts = parse_hosts(&hosts).expect("Invalid config file");
    //match sub commands
    match args.sub_cmd {
        SubCommand::Switch(s) => {
            println!("switching")
        },
    }
    println!("{:?}", hosts);
}