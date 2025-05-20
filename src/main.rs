use clap::Parser;
use console::{style, Term};
use crufty::cli::{Cli, Commands};
use crufty::estimator::estimate;
use crufty::fetcher::fetch_artifacts;
use crufty::types::Size;
use crufty::ui;
use std::env;
use std::io;
use std::process;

mod crufty;

fn main() {
  let cli = Cli::parse();

  match &cli.command {
    Commands::Scan => match scan() {
      Err(err) => exit_unrecoverable(err),
      Ok(_) => {}
    },
  }
}

fn scan() -> io::Result<()> {
  let term = Term::stdout();
  let path = env::current_dir()?;
  term
    .write_line(&format!("[+] Scanning: {}", style(path.display()).bold()))?;
  term.write_line("")?;

  // Fetch artifacts
  let mut artifacts = fetch_artifacts(&path);

  if artifacts.is_empty() {
    term.write_line("No significant build artifacts found.")
  } else {
    let pb = ui::create_progress_bar(artifacts.len() as u64);

    for artifact in artifacts.iter_mut() {
      estimate(artifact);
      pb.inc(1);
    }
    pb.finish_and_clear();

    for (i, artifact) in artifacts.iter().enumerate() {
      let rel_path =
        artifact.path.strip_prefix(&path).unwrap_or(&artifact.path);
      let size_display = match &artifact.size {
        Size::UnknownSize => style(format!("{}", artifact.size)).yellow(),
        Size::KnownSize(_) => style(format!("{}", artifact.size)).green(),
      };

      term.write_line(&format!(
        "[{}] {:<30} {}",
        i + 1,
        style(format!("./{}", rel_path.display())).bold(),
        size_display
      ))?;
    }

    term.write_line("")?;
    term.write_line(&format!(
      "{} Scan completed successfully",
      style("✓").green()
    ))?;

    Ok(())
  }
}

fn exit_unrecoverable(_err: io::Error) {
  let term_err = Term::stdout();
  let error_message = "Encountered error, existing...";
  let _ = term_err.write_line(&format!("{}", error_message));
  process::exit(1);
}
