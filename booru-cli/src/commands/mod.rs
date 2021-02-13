use structopt::{clap, StructOpt};
use booru::error::BooruError;

pub mod progress;

pub mod login;
pub mod fetch;
pub mod fetch_all;
pub mod resize;

#[derive(Debug, StructOpt)]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
/// CLI application to access Danbooru API.
pub struct BooruOpt {
    #[structopt(subcommand)]
    pub subcommand: SubCommand,
}


#[derive(Debug, StructOpt)]
pub enum SubCommand {
    /// Login to danbooru
    Login,
    /// Download posts to your local computer by selecting the tags and pages.
    Fetch(fetch::TagDownloadOpt),
    /// Download posts to your local computer by selecting the tags and multiple pages.
    FetchAll(fetch_all::FetchAllDownloadOpt),
    Resize(resize::ResizeOpt),
}

impl BooruOpt {
    pub fn run_subcommand(&self) -> Result<(), BooruError>{
        use SubCommand::*;
        match &self.subcommand {
            Login => login::login(),
            Fetch(opt) => fetch::download(opt),
            FetchAll(opt) => fetch_all::download(opt),
            Resize(opt) => resize::resize(opt),
        }
    }
}