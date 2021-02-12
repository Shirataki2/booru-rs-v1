use structopt::{clap, StructOpt};
use booru::{
    error::BooruError,
    http::BooruClient,
    config::Config,
    api::posts::{Posts, PostsFetchRequestBuilder},
};
use threadpool::ThreadPool;
use std::path::PathBuf;
use colored::*;
use std::sync::Arc;
use std::fs;
use crate::commands::utils::Spinner;

#[derive(Debug, StructOpt)]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct TagDownloadOpt {
    #[structopt(short)]
    pub tags: Vec<String>,
    #[structopt(short = "j", default_value = "1")]
    pub num_threads: usize,
    #[structopt(short, default_value = "1")]
    pub page: u64,
    #[structopt(short = "n", default_value = "20")]
    pub limit: u64,
    #[structopt()]
    pub out_dir: PathBuf,
}

pub fn download(opt: &TagDownloadOpt) -> Result<(), BooruError> {
    let conf = Config::load()?;
    let query = PostsFetchRequestBuilder::default()
        .tags(opt.tags.clone())
        .page(opt.page)
        .limit(opt.limit)
        .build()
        .unwrap();
    let posts = BooruClient::from_config(&conf)?.get_posts(query)?;
    fs::create_dir_all(opt.out_dir.clone())?;
    match opt.num_threads {
        1 => {
            for post in posts {
                let filename = format!("{}.{}", post.id, post.file_ext);
                match post.save(&opt.out_dir) {
                    Ok(()) => eprintln!("[{}] Successfully saved {}", "INFO".green(), filename.bold()),
                    Err(_) => eprintln!("[{}] Failed to save {}", "Error".red(), filename.bold()),
                };
            }
        },
        n => {
            let n = if n == 0 { num_cpus::get() } else { n };
            let pool = ThreadPool::new(n);
            let spinner = Spinner::start("Saving...");

            let path = Arc::new(opt.out_dir.clone());

            for post in posts {
                let path = Arc::clone(&path);
                pool.execute(move || {
                    let _ = post.save(&path);
                })
            }
            pool.join();
            spinner.stop("Success!");
        },
    };
    Ok(())
}