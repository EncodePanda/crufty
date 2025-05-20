use indicatif::{ProgressBar, ProgressStyle};

/// Creates and returns a configured progress bar for use in the application.
pub fn create_progress_bar(total: u64) -> ProgressBar {
  let pb = ProgressBar::new(total);
  let template =
	"{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}";
  let style = ProgressStyle::default_bar()
    .template(template)
    .unwrap()
    .progress_chars("#>-");
  pb.set_style(style);
  pb
}
