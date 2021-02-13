use structopt::{clap, StructOpt};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use booru::error::BooruError;
use std::fs;
use image::imageops;
use colored::*;
use threadpool::ThreadPool;
use crate::commands::progress::FetchProgressBar;

#[derive(Debug, StructOpt)]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct ResizeOpt {
    #[structopt(short)]
    /// Directory
    pub source_dir: PathBuf,

    #[structopt(short)]
    /// Directory
    pub target_dir: PathBuf,

    #[structopt(short)]
    pub width: u32,

    #[structopt(short = "j", default_value = "4")]
    /// Number of parallel threads
    pub jobs: usize,
}

pub fn resize(opt: &ResizeOpt) -> Result<(), BooruError> {
    let (src, dst) = (opt.source_dir.clone(), opt.target_dir.clone());
    if !src.is_dir() {
        return Err(BooruError::PathIsNotDirectory(src.to_str().unwrap().to_owned()))
    }
    fs::create_dir_all(&dst)?;
    
    let mut images = Vec::new();
    for file in fs::read_dir(src)? {
        let path = file?.path();
        let is_image = &["jpg", "png", "jpeg"].iter().any(|&ext| {
            path.extension().unwrap().to_str().unwrap() == ext
        });
        if !path.is_dir() && *is_image {
            images.push(path);
        }
    }
    eprintln!("[{}] {}", "*".green(), (&format!("{} images found", images.len())).bold());

    let size = opt.width;

    let dst = Arc::new(dst);

    let pb = Arc::new(Mutex::new(FetchProgressBar::start(images.len() as u64)));
    let pool = ThreadPool::new(opt.jobs);
    for im in images {
        let dst = Arc::clone(&dst);
        let pb = Arc::clone(&pb);
        pool.execute(move || {
            let img = image::open(im.clone()).unwrap();
            let img = imageops::resize(&img, size, size, imageops::FilterType::Triangle);
            let filename = im.file_name().unwrap();
            let path = dst.join(filename);
            let _ = img.save(&path);
            pb.lock().unwrap().add();
        });
    }
    pool.join();
    pb.lock().unwrap().stop("Finish!");
    Ok(())
}