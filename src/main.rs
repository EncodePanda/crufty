use clap::Parser;
use console::{style, Term};
use crufty::cleaner::{self, CleanupResult};
use crufty::cli::{Cli, Commands};
use crufty::estimator::{estimate, total_size};
use crufty::fetcher::fetch_artifacts;
use crufty::types::Size;
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
    Commands::Clean => match clean() {
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

  let spinner = ui::create_spinner("collecting artifacts");
  let mut artifacts = fetch_artifacts(&path);
  spinner.finish_and_clear();

  term.write_line("")?;

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

    term.write_line("Use `crufty clean` to remove these safely")?;

    Ok(())
  }
}

fn clean() -> io::Result<()> {
  let term = Term::stdout();
  let path = env::current_dir()?;

  let spinner = ui::create_spinner("collecting artifacts");
  let mut artifacts = fetch_artifacts(&path);
  spinner.finish_and_clear();

  if artifacts.is_empty() {
    term.write_line("No significant build artifacts found.")
  } else {
    let pb = ui::create_progress_bar(artifacts.len() as u64);

    let mut sizes: Vec<&Size> = vec![];

    for artifact in artifacts.iter_mut() {
      estimate(artifact);
      match cleaner::clean(&artifact) {
        CleanupResult::Successful => sizes.push(&artifact.size),
        CleanupResult::DoesNotExists => {
          let term_err = Term::stderr();
          term_err.write_line(&format!(
            "Unable to remove {:?} ({}), already removed",
            artifact.path,
            style_size(&artifact.size),
          ))?;
        }
        CleanupResult::Unsuccessful => {
          let term_err = Term::stderr();
          term_err.write_line(&format!(
            "Unable to remove {:?} ({})",
            artifact.path,
            style_size(&artifact.size),
          ))?;
        }
      }
      pb.inc(1);
    }
    pb.finish_and_clear();

    let total = total_size(sizes.as_slice());

    term.write_line("")?;
    term.write_line(&format!(
      "{} directories were removed, restoring {}",
      style(format!("{}", sizes.len())).bold(),
      style_size(&total),
    ))?;
    Ok(())
  }
}

fn exit_unrecoverable(_err: io::Error) {
  let term_err = Term::stderr();
  let error_message = "Encountered error, existing...";
  let _ = term_err.write_line(&format!("{}", error_message));
  process::exit(1);
}
