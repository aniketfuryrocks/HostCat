use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "Aniket Prajapati <prajapati.ani306@gmail.com>")]
pub struct Args {
    #[clap(short = "c", long = "config", default_value = "/etc/hosts")]
    pub config: String,
    #[clap(long = "ru", about = "continue without checking for root privileges")]
    pub root_unchecked: bool,
}

pub fn map_args() -> Args {
    Args::parse()
}