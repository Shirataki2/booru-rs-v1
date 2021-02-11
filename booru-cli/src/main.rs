#![warn(clippy::all)]
mod commands;

use structopt::StructOpt;
use commands::BooruOpt;

fn main() -> anyhow::Result<()> {
    let opt = BooruOpt::from_args();
    opt.run_subcommand()?;
    Ok(())
}
