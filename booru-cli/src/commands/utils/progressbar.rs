use indicatif::{ProgressBar, ProgressStyle};

pub struct Spinner(ProgressBar);

impl Spinner {
    pub fn start(message: &str) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(50);
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.blue} {msg}")
        );
        pb.set_message(message);
        Self(pb)
    }

    pub fn stop(&self, message: &str) {
        self.0.finish_with_message(message);
    }
}
