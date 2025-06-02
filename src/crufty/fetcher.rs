use std::path::PathBuf;

use super::artifact_type::ArtifactType;
use globset::{Glob, GlobSet, GlobSetBuilder};
use walkdir::WalkDir;

use super::types::ArtifactCandidate;

fn mk_global_set(
  artifact_types: Vec<ArtifactType>,
) -> Result<GlobSet, globset::Error> {
  let mut builder = GlobSetBuilder::new();
  for art_type in artifact_types {
    builder.add(Glob::new(art_type.pattern())?);
  }
  builder.build()
}

pub fn fetch_artifacts(
  root_path: &PathBuf,
  artifact_types: Vec<ArtifactType>,
) -> Vec<ArtifactCandidate> {
  match mk_global_set(artifact_types) {
    Err(_) => vec![],
    Ok(globset) => WalkDir::new(root_path)
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
      .collect(),
  }
}

#[cfg(test)]
mod tests {

  use assert_fs::{
    fixture::{ChildPath, PathChild},
    prelude::*,
    TempDir,
  };

  use crate::crufty::artifact_type::ArtifactType;

  use super::{fetch_artifacts, ArtifactCandidate};

  fn mk_subpath(base: &TempDir, rel_path: &str) -> ChildPath {
    let sub = base.child(rel_path);
    sub.create_dir_all().unwrap();
    sub
  }

  fn only_rust_projects() -> Vec<ArtifactType> {
    vec![ArtifactType::Rust]
  }

  fn custom_project(pattern: &'static str) -> Vec<ArtifactType> {
    vec![ArtifactType::Custom { pattern }]
  }

  fn mk_rust_project<P: PathChild>(base: &P) {
    base.child("target").create_dir_all().unwrap();
    base.child("Cargo.toml").touch().unwrap();
  }

  #[test]
  fn test_simple_rust_project_being_scanned_folder() {
    // given there is a single rust project in the folder
    let temp = TempDir::new().unwrap();
    mk_rust_project(&temp);

    // when we search for Rust artifacts
    let results = fetch_artifacts(&temp.to_path_buf(), only_rust_projects());

    // then
    assert_eq!(results.len(), 1, "Expected exactly one artifact");

    let expected_path = temp.child("target").path().to_path_buf();
    let expected = ArtifactCandidate::new(expected_path);
    assert_eq!(&results[0], &expected);

    // when we search for projects other than Rust
    let results =
      fetch_artifacts(&temp.to_path_buf(), custom_project("**/bla"));
    // then
    assert_eq!(results.len(), 0, "Expected zero artifacts");

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

    // when we search for Rust artifacts
    let mut results =
      fetch_artifacts(&temp.to_path_buf(), only_rust_projects());

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

    // when we search for projects other than Rust
    let results =
      fetch_artifacts(&temp.to_path_buf(), custom_project("**/bla"));
    // then
    assert_eq!(results.len(), 0, "Expected zero artifacts");

    temp.close().unwrap();
  }
}
