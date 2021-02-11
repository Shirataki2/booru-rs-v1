use structopt::{clap, StructOpt};
use booru::error::BooruError;

pub mod login;
pub mod utils;
pub mod tag;

#[derive(Debug, StructOpt)]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct BooruOpt {
    #[structopt(subcommand)]
    pub subcommand: SubCommand,
}


#[derive(Debug, StructOpt)]
pub enum SubCommand {
    Login,
    Tag(tag::TagDownloadOpt)
}

impl BooruOpt {
    pub fn run_subcommand(&self) -> Result<(), BooruError>{
        use SubCommand::*;
        match &self.subcommand {
            Login => login::login(),
            Tag(opt) => tag::download(opt),
        }
    }
}