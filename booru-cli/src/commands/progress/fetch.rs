use indicatif::{ProgressBar, ProgressStyle};

pub struct FetchProgressBar(ProgressBar, u64);

impl FetchProgressBar {
    pub fn start(length: u64) -> Self {
        let pb = ProgressBar::new(length);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.blue} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {per_sec} eta:{eta_precise} {msg}")
                .progress_chars("=>-")
        );
        Self(pb, 0)
    }

    pub fn add(&mut self) {
        self.1 += 1;
        self.0.set_position(self.1);
    }

    pub fn stop(&self, message: &str) {
        self.0.finish_with_message(message);
    }
}