use structopt::{clap, StructOpt};
use booru::{
    error::BooruError,
    http::BooruClient,
    config::Config,
    api::posts::{Posts, PostsFetchRequestBuilder},
};
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::fs;
use colored::*;
use crate::commands::progress::{DownloadProgressBar, FetchProgressBar};

const PAGE_LIMIT: u64 = 100;

#[derive(Debug, StructOpt)]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct FetchAllDownloadOpt {
    #[structopt(short)]
    /// Tags (https://danbooru.donmai.us/wiki_pages/help%3Acheatsheet)
    pub tags: Vec<String>,

    #[structopt(short = "x")]
    /// Tags (https://danbooru.donmai.us/wiki_pages/help%3Acheatsheet)
    pub exclude_tags: Vec<String>,

    #[structopt(short = "j", default_value = "1")]
    /// Number of parallel threads
    pub jobs: usize,

    #[structopt(short, default_value = "1")]
    /// Search page
    pub start_page: u64,

    #[structopt(short, default_value = "5")]
    /// Search page
    pub end_page: u64,

    #[structopt(short = "n", default_value = "20")]
    /// Number of posts per page
    pub limit: u64,

    #[structopt(short, default_value = "1.3")]
    /// Search page
    pub ratio_limit: f64,

    #[structopt(short)]
    /// Directory for saving images
    pub out_dir: PathBuf,
}

pub fn download(opt: &FetchAllDownloadOpt) -> Result<(), BooruError> {
    let mut tags = opt.tags.clone();
    for tag in opt.exclude_tags.iter() {
        tags.push(format!("-{}", tag));
    }
    let tags = Arc::new(tags);
    let posts = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(BooruClient::from_config(&Config::load()?)?);
    
    let (s, mut e) = (opt.start_page, opt.end_page);
    if e < s {
        panic!("Invalid arguments");
    }
    if e - s > PAGE_LIMIT {
        println!("The maximum number of pages that can be downloaded at one time is {}.", PAGE_LIMIT);
        e = s + PAGE_LIMIT;
    }
    let n = opt.jobs;
    let n = if n == 0 { num_cpus::get() } else { n };

    let limit = opt.limit;

    println!("[{}] {}", "*".green(), "Downloading Page Data".bold());

    let pb = Arc::new(Mutex::new(FetchProgressBar::start(e-s+1)));
    let pool = ThreadPool::new(n);

    for p in s..=e {
        let tags = Arc::clone(&tags);
        let posts = Arc::clone(&posts);
        let client = Arc::clone(&client);
        let pb = Arc::clone(&pb);
        pool.execute(move || {
            let tags = &*tags;
            let query = PostsFetchRequestBuilder::default()
                .tags(tags.clone())
                .page(p)
                .limit(limit)
                .build()
                .unwrap();
            let page_posts = match client.get_posts(query) {
                Ok(page_posts) => page_posts,
                Err(_) => {
                    eprintln!("[{}] {} {}\n", "*".red(), "Failed to download page:".bold(), p);
                    pb.lock().unwrap().add();
                    return;
                }
            };
            for post in page_posts {
                posts.lock().unwrap().push(post);
            }
            pb.lock().unwrap().add();
        });
        thread::sleep(Duration::from_millis(200));
    }
    pool.join();
    pb.lock().unwrap().stop("Finish!");
    let posts = posts.lock().unwrap().clone();
    eprintln!("[{}] {}", "*".green(), (&format!("{} post(s) downloaded!", posts.len())).bold());
    fs::create_dir_all(opt.out_dir.clone())?;

    println!("[{}] {}", "*".green(), "Downloading Post Data".bold());

    let pool = ThreadPool::new(n);
    let path = Arc::new(opt.out_dir.clone());

    let pb = Arc::new(Mutex::new(DownloadProgressBar::start(&posts)));

    let ctr = Arc::new(AtomicUsize::new(0usize));
    let rl = opt.ratio_limit;
    for post in posts {
        let path = Arc::clone(&path);
        let pb = Arc::clone(&pb);
        let ctr = Arc::clone(&ctr);
        pool.execute(move || {
            if post.save_ratio(&path, rl).is_ok() { ctr.fetch_add(1, Ordering::SeqCst); }
            pb.lock().unwrap().add(&post);
        })
    }
    pool.join();
    let c = ctr.fetch_add(0, Ordering::SeqCst);
    pb.lock().unwrap().stop("Finish!");
    eprintln!("[{}] {}", "*".green(), (&format!("{} image(s) saved!", c)).bold());
    Ok(())
}