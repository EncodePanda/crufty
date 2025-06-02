use console::{style, StyledObject};
use indicatif::{ProgressBar, ProgressStyle};

use super::types::Size;

/// Creates and returns a configured progress bar for use in the application.
pub fn create_progress_bar(total: u64) -> ProgressBar {
  let pb = ProgressBar::new(total);
  let template = "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}";
  let bar_style = ProgressStyle::default_bar()
    .template(template)
    .unwrap()
    .progress_chars("#>-");
  pb.set_style(bar_style);
  pb
}

/// Creates and returns a spinner for showing progress on a single operation.
pub fn create_spinner(message: &str) -> ProgressBar {
  let sp = ProgressBar::new_spinner();
  let spinner_style =
    ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
      .unwrap()
      .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈");
  sp.set_style(spinner_style);
  sp.set_message(message.to_string());
  sp.enable_steady_tick(std::time::Duration::from_millis(120));
  sp
}

pub fn style_size(size: &Size) -> StyledObject<String> {
  let mb = 1024 * 1024;
  match size {
    Size::UnknownSize => style(format!("{}", size)).red(),
    Size::KnownSize(bytes) if (*bytes < 5 * mb) => {
      style(format!("{}", size)).green()
    }
    Size::KnownSize(bytes) if (*bytes < 250 * mb) => {
      style(format!("{}", size)).yellow()
    }
    Size::KnownSize(_) => style(format!("{}", size)).red(),
  }
}
