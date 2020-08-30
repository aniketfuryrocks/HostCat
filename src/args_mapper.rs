use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "Aniket Prajapati <prajapati.ani306@gmail.com>")]
pub struct Args {
    #[clap(short = "f", long = "file", default_value = "/etc/hosts")]
    pub file: String,
    #[clap(long = "ru", about = "continue without checking for root privileges")]
    pub root_unchecked: bool,
    #[clap(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap()]
    Switch(Switch)
}

#[derive(Clap)]
pub struct Switch;

pub fn map_args() -> Args {
    Args::parse()
}