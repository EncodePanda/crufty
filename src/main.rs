use clap::Parser;
use console::{style, Term};
use crufty::cli::{Cli, Commands};
use crufty::fetch_artifacts;
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
  let artifacts = fetch_artifacts(&path);

  if artifacts.is_empty() {
    term.write_line("No significant build artifacts found.")
  } else {
    for (i, artifact) in artifacts.iter().enumerate() {
      let rel_path =
        artifact.path.strip_prefix(&path).unwrap_or(&artifact.path);
      term.write_line(&format!(
        "[{}] {:<30} {}",
        i + 1,
        style(format!("./{}", rel_path.display())).bold(),
        style(format!("unknown size")).yellow()
      ))?;
    }
    Ok(())
  }
}

fn exit_unrecoverable(_err: io::Error) {
  let term_err = Term::stdout();
  let error_message = "Encountered error, existing...";
  let _ = term_err.write_line(&format!("{}", error_message));
  process::exit(1);
}
