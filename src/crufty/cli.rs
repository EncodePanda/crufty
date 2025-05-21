use clap::{Parser, Subcommand};

/// A command-line tool that scans projects for large build artifacts and cleans
/// them up
#[derive(Parser, Debug, PartialEq)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
  /// Scan for build artifacts in the current directory
  Scan,
  /// Clean all build artifacts in the current directory
  Clean,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parsing_scan_command() {
    let args = vec!["crufty", "scan"];
    let cli = Cli::parse_from(args);

    assert_eq!(
      cli,
      Cli {
        command: Commands::Scan
      }
    );
  }
}
