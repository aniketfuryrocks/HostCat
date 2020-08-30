use crate::parser::{parse_hosts, read_hosts};

mod parser;
mod privileges;

fn main() {
    //check if root
    //privileges::check_privileges();
    //read hosts
    let hosts = read_hosts();
    let hosts = parse_hosts(&hosts).unwrap();
    println!("{:?}", hosts);
}