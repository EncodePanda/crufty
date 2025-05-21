use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use super::types::{ArtifactCandidate, Size};

pub trait Sizable {
  fn get_size(&self) -> &Size;
  fn bytes(&self) -> &u64 {
    match self.get_size() {
      Size::UnknownSize => &0,
      Size::KnownSize(b) => b,
    }
  }
}

impl Sizable for ArtifactCandidate {
  fn get_size(&self) -> &Size {
    &self.size
  }
}

impl Sizable for Size {
  fn get_size(&self) -> &Size {
    &self
  }
}

impl Sizable for &Size {
  fn get_size(&self) -> &Size {
    self
  }
}

pub fn total_size(sizables: &[impl Sizable]) -> Size {
  let total = sizables
    .iter()
    .fold(0, |acc, artifact| acc + artifact.bytes());
  Size::KnownSize(total)
}

pub fn estimate(artifact: &mut ArtifactCandidate) -> &ArtifactCandidate {
  let size = estimate_path(&artifact.path);
  artifact.size = size;
  artifact
}

fn estimate_path(path: &PathBuf) -> Size {
  match calculate_dir_size(&path) {
    Ok(size) => Size::KnownSize(size),
    Err(_) => Size::UnknownSize,
  }
}

fn calculate_dir_size(path: &PathBuf) -> std::io::Result<u64> {
  let mut total = 0;

  for entry in WalkDir::new(path)
    .follow_links(false)
    .into_iter()
    .filter_map(Result::ok)
    .filter(|e| e.file_type().is_file())
  {
    if let Ok(metadata) = fs::metadata(entry.path()) {
      total += metadata.len();
    }
  }

  Ok(total)
}

#[cfg(test)]
mod tests {
  use super::*;
  use assert_fs::prelude::*;
  use assert_fs::TempDir;

  #[test]
  fn test_estimate_path_empty_dir() {
    // given
    let temp = TempDir::new().unwrap();
    let path = temp.path().to_path_buf();
    let mut artifact = ArtifactCandidate::new(path);
    // when
    let artifact = estimate(&mut artifact);
    // then
    assert!(matches!(artifact.size, Size::KnownSize(0)));

    temp.close().unwrap();
  }

  #[test]
  fn test_estimate_path_with_files() {
    // given
    let temp = TempDir::new().unwrap();
    let file = temp.child("test.txt");
    file.write_str("Hello, world!").unwrap();
    let path = temp.path().to_path_buf();
    let mut artifact = ArtifactCandidate::new(path);
    // when
    let artifact = estimate(&mut artifact);
    // then
    assert!(matches!(artifact.size, Size::KnownSize(13)));

    temp.close().unwrap();
  }
}
