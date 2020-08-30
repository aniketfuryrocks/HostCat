use crate::args_mapper::map_args;
use crate::parser::{parse_hosts, read_hosts};

mod parser;
mod privileges;
mod args_mapper;

fn main() {
    let args = map_args();
    if !args.root_unchecked {
        privileges::check_privileges();
    }
    let hosts = read_hosts(&args.config);
    let hosts = parse_hosts(&hosts).expect("Invalid config file");
    println!("{:?}", hosts);
}