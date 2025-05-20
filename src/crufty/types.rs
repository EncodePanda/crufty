#![allow(dead_code)]
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Size {
  UnknownSize,
  KnownSize(u64),
}

impl fmt::Display for Size {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Size::UnknownSize => write!(f, "unknown"),
      Size::KnownSize(bytes) => {
        if *bytes < 1024 {
          write!(f, "{} B", bytes)
        } else if *bytes < 1024 * 1024 {
          write!(f, "{:.1} KB", *bytes as f64 / 1024.0)
        } else if *bytes < 1024 * 1024 * 1024 {
          write!(f, "{:.1} MB", *bytes as f64 / (1024.0 * 1024.0))
        } else {
          write!(f, "{:.1} GB", *bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        }
      }
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArtifactCandidate {
  pub path: PathBuf,
  pub size: Size,
}

impl ArtifactCandidate {
  pub fn new(path: PathBuf) -> Self {
    ArtifactCandidate {
      path,
      size: Size::UnknownSize,
    }
  }
}

impl Ord for ArtifactCandidate {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.path.cmp(&other.path)
  }
}

impl PartialOrd for ArtifactCandidate {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}
