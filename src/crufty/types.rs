use crate::crufty::artifact_type::ArtifactType;
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArtifactCandidate {
  pub path: PathBuf,
  pub size: Size,
  pub art_type: Option<ArtifactType>,
}

pub struct ArtifactCandidateBuilder {
  path: PathBuf,
  size: Size,
  art_type: Option<ArtifactType>,
}

impl ArtifactCandidateBuilder {
  pub fn new(path: PathBuf) -> Self {
    ArtifactCandidateBuilder {
      path,
      size: Size::UnknownSize,
      art_type: None,
    }
  }

  pub fn art_type(mut self, art_type: Option<ArtifactType>) -> Self {
    self.art_type = art_type;
    self
  }

  pub fn build(self) -> ArtifactCandidate {
    ArtifactCandidate {
      path: self.path,
      size: self.size,
      art_type: self.art_type,
    }
  }
}

impl ArtifactCandidate {
  pub fn builder(path: PathBuf) -> ArtifactCandidateBuilder {
    ArtifactCandidateBuilder::new(path)
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
