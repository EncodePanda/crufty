use clap::{Parser, Subcommand};

/// A command-line tool that scans projects for large build artifacts and cleans
/// them up
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Scan for build artifacts in the current directory
  Scan,
}
