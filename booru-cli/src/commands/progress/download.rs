use indicatif::{ProgressBar, ProgressStyle};
use booru::models::Post;

pub struct DownloadProgressBar(ProgressBar, u64);

impl DownloadProgressBar {
    pub fn start(posts: &[Post]) -> Self {
        let total_bytes = posts.iter().fold(0, |acc, post| {
            acc + post.file_size
        });
        let pb = ProgressBar::new(total_bytes);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.blue} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} {bytes_per_sec} eta:{eta_precise} {msg}")
                .progress_chars("=>-")
        );
        Self(pb, 0)
    }

    pub fn add(&mut self, post: &Post) {
        let bytes = post.file_size;
        self.1 += bytes;
        self.0.set_position(self.1);
    }

    pub fn stop(&self, message: &str) {
        self.0.finish_with_message(message);
    }
}