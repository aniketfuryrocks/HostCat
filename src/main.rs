use crate::args_mapper::map_args;
use crate::parser::{parse_hosts, read_hosts};

mod parser;
mod privileges;
mod args_mapper;

fn main() {
    privileges::check_privileges();
    let args = map_args();
    let hosts = read_hosts(&args.config);
    let hosts = parse_hosts(&hosts).expect("Invalid config file");
    println!("{:?}", hosts);
}