use clap::Parser;
use console::{style, Term};
use crufty::cli::{Cli, Commands};
use crufty::estimator::{estimate, total_size};
use crufty::fetcher::fetch_artifacts;
use crufty::ui::{self, style_size};
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

      term.write_line(&format!(
        "[{}] {:<36} {}",
        i + 1,
        style(format!("./{}", rel_path.display())).bold(),
        style_size(&artifact.size)
      ))?;
    }

    let total = total_size(&artifacts);

    term.write_line("")?;
    term.write_line(&format!(
      "Total size: {} in {} directories",
      style_size(&total),
      style(format!("{}", artifacts.len())).bold()
    ))?;

    term.write_line("Use `crafty clean` to remove these safely")?;

    Ok(())
  }
}

fn exit_unrecoverable(_err: io::Error) {
  let term_err = Term::stdout();
  let error_message = "Encountered error, existing...";
  let _ = term_err.write_line(&format!("{}", error_message));
  process::exit(1);
}
