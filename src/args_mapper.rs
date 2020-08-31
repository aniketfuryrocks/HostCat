use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "Aniket Prajapati <prajapati.ani306@gmail.com>")]
pub struct Args {
    #[clap(short = "c", long = "config", default_value = "/etc/hostcat")]
    pub config: String,
    #[clap(long = "ru", about = "continue without checking for root privileges")]
    pub root_unchecked: bool,
    #[clap(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(about = "switch to profile specified by -p option")]
    Switch(SwitchArg),
    #[clap(about = "create/update/delete a profile")]
    Set(SetArg),
}


#[derive(Clap)]
pub struct SwitchArg {
    #[clap(short = "f", long = "file", default_value = "/etc/hosts", about = "Path to host file")]
    pub file: String,
    #[clap(short = "p", long = "profile", default_value = "default", about = "Profile to switch to")]
    pub profile: String,
}

#[derive(Clap)]
pub struct SetArg {
    #[clap(short = "p", long = "profile", default_value = "default", about = "Profile to switch to")]
    pub profile: String,
    #[clap(short = "v", long = "value", about = "pass space in \" to delete profile, space separated values in \" for multiple values ")]
    pub value: String,
}

pub fn map_args() -> Args {
    Args::parse()
}