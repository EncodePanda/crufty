#![allow(dead_code)]
use std::path::PathBuf;

pub mod cli;

#[derive(Debug, PartialEq)]
pub struct ArtifactCandidate {
  pub path: PathBuf,
  size: Option<u64>,
}

impl ArtifactCandidate {
  fn new(path: PathBuf) -> Self {
    ArtifactCandidate { path, size: None }
  }
}

pub fn fetch_artifacts(_path: &PathBuf) -> Vec<ArtifactCandidate> {
  vec![]
}

#[cfg(test)]
mod tests {

  use assert_fs::prelude::*;
  use assert_fs::{
    fixture::{ChildPath, PathChild},
    TempDir,
  };

  use super::{fetch_artifacts, ArtifactCandidate};

  fn mk_subpath(base: &TempDir, rel_path: &str) -> ChildPath {
    let sub = base.child(rel_path);
    sub.create_dir_all().unwrap();
    sub
  }

  fn mk_rust_project<P: PathChild>(base: &P) {
    base.child("target").create_dir_all().unwrap();
    base.child("Cargo.toml").touch().unwrap();
  }

  #[test]
  #[ignore]
  fn test_simple_rust_project_being_scanned_folder() {
    // given
    let temp = TempDir::new().unwrap();
    mk_rust_project(&temp);

    // when
    let results = fetch_artifacts(&temp.to_path_buf());

    // then
    assert_eq!(results.len(), 1, "Expected exactly one artifact");

    let expected_path = temp.child("target").path().to_path_buf();
    let expected = ArtifactCandidate::new(expected_path);
    assert_eq!(&results[0], &expected);

    temp.close().unwrap();
  }
}
