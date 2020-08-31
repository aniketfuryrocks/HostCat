use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.2.0", about = "UNIX Command Line tool to switch between Local Dns profiles")]
pub struct Args {
    #[clap(short = "c", long = "config", default_value = "~/.hostcat")]
    pub config: String,
    #[clap(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(about = "switch to profile specified by -p option")]
    Switch(SwitchArg),
    #[clap(about = "create/update/delete a profile")]
    Set(SetArg),
    #[clap(about = "list all profiles")]
    Profiles(EmptyCmd),
}

#[derive(Clap)]
pub struct EmptyCmd;

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