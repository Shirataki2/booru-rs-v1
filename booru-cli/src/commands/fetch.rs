use structopt::{clap, StructOpt};
use booru::{
    error::BooruError,
    http::BooruClient,
    config::Config,
    api::posts::{Posts, PostsFetchRequestBuilder},
};
use threadpool::ThreadPool;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::fs;
use crate::commands::progress::DownloadProgressBar;

#[derive(Debug, StructOpt)]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct TagDownloadOpt {
    #[structopt(short)]
    /// Tags (https://danbooru.donmai.us/wiki_pages/help%3Acheatsheet)
    pub tags: Vec<String>,

    #[structopt(short = "j", default_value = "1")]
    /// Number of parallel threads
    pub jobs: usize,

    #[structopt(short, default_value = "1")]
    /// Search page
    pub page: u64,

    #[structopt(short = "n", default_value = "20")]
    /// Number of posts per page
    pub limit: u64,

    #[structopt(short)]
    /// Directory for saving images
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
    let n = opt.jobs;
    let n = if n == 0 { num_cpus::get() } else { n };
    let pool = ThreadPool::new(n);
    let path = Arc::new(opt.out_dir.clone());

    let pb = Arc::new(Mutex::new(DownloadProgressBar::start(&posts)));

    for post in posts {
        let path = Arc::clone(&path);
        let pb = Arc::clone(&pb);
        pool.execute(move || {
            let _ = post.save(&path);
            pb.lock().unwrap().add(&post);
        })
    }
    pool.join();
    pb.lock().unwrap().stop("Finish!");
    Ok(())
}