use std::path::PathBuf;

use globset::{Glob, GlobSetBuilder};
use walkdir::WalkDir;

use super::types::ArtifactCandidate;

pub fn fetch_artifacts(root_path: &PathBuf) -> Vec<ArtifactCandidate> {
  let mut builder = GlobSetBuilder::new();
  // FIX-ME hardcoded pattern for Rust
  builder.add(Glob::new("**/target").unwrap());

  let globset = builder.build().unwrap();

  WalkDir::new(root_path)
    .into_iter()
    .filter_map(|entry_result| match entry_result {
      Err(_) => None,
      Ok(entry) if !entry.path().is_dir() => None,
      Ok(entry) => {
        let path = entry.into_path();
        let rel_path = path.strip_prefix(root_path).ok()?;
        match globset.is_match(&rel_path) {
          true => Some(ArtifactCandidate::new(path)),
          false => None,
        }
      }
    })
    .collect()
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

  #[test]
  fn test_where_scanned_folder_has_three_rust_projects() {
    // given
    let temp = TempDir::new().unwrap();
    let project1 = mk_subpath(&temp, "project1");
    mk_rust_project(&project1);
    let project2 = mk_subpath(&temp, "project2");
    mk_rust_project(&project2);
    let project3 = mk_subpath(&temp, "work/project3");
    mk_rust_project(&project3);

    // when
    let mut results = fetch_artifacts(&temp.to_path_buf());

    // then
    assert_eq!(results.len(), 3, "Expected exactly three artifacts");
    results.sort();

    let expected_path_1 =
      temp.child("project1").child("target").path().to_path_buf();
    let expected_1 = ArtifactCandidate::new(expected_path_1);
    assert_eq!(&results[0], &expected_1);

    let expected_path_2 =
      temp.child("project2").child("target").path().to_path_buf();
    let expected_2 = ArtifactCandidate::new(expected_path_2);
    assert_eq!(&results[1], &expected_2);

    let expected_path_3 = temp
      .child("work")
      .child("project3")
      .child("target")
      .path()
      .to_path_buf();
    let expected_3 = ArtifactCandidate::new(expected_path_3);
    assert_eq!(&results[2], &expected_3);

    temp.close().unwrap();
  }
}
