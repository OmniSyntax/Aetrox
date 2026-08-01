// 👈 (For loading spinners & colors)
use indicatif::{ProgressBar, ProgressStyle};

pub fn show_spinner(message: &str) -> ProgressBar {
    // ... paste your loading spinner config here ...
    ProgressBar::new_spinner()
}