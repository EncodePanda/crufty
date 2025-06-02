use std::path::PathBuf;

use super::artifact_type::ArtifactType;
use globset::{Glob, GlobSet, GlobSetBuilder};
use walkdir::WalkDir;

use super::types::ArtifactCandidate;

fn mk_global_set(
  artifact_types: &Vec<ArtifactType>,
) -> Result<GlobSet, globset::Error> {
  let mut builder = GlobSetBuilder::new();
  for art_type in artifact_types {
    builder.add(Glob::new(art_type.pattern())?);
  }
  builder.build()
}

fn detect_artifact_type(
  parent_path: &PathBuf,
  artifact_types: &[ArtifactType],
) -> Option<ArtifactType> {
  match artifact_types {
    [] => None,
    [head, tail @ ..] => {
      let recognized_files = head.recognized_files();
      let all_files_present = recognized_files.iter().all(|file| {
        let file_path = parent_path.join(file);
        file_path.exists()
      });

      match all_files_present {
        true => Some(head.clone()),
        false => detect_artifact_type(parent_path, tail),
      }
    }
  }
}

pub fn fetch_artifacts(
  root_path: &PathBuf,
  artifact_types: &Vec<ArtifactType>,
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
            true => {
              let parent_path = path.parent()?;
              let art_type = detect_artifact_type(
                &parent_path.to_path_buf(),
                &artifact_types,
              );
              let candidate =
                ArtifactCandidate::builder(path).art_type(art_type).build();
              Some(candidate)
            }
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
    vec![ArtifactType::Custom {
      pattern,
      name: "Custom",
      files: &[],
    }]
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
    let results = fetch_artifacts(&temp.to_path_buf(), &only_rust_projects());

    // then
    assert_eq!(results.len(), 1, "Expected exactly one artifact");

    let expected_path = temp.child("target").path().to_path_buf();
    let expected = ArtifactCandidate::builder(expected_path)
      .art_type(Some(ArtifactType::Rust))
      .build();
    assert_eq!(&results[0], &expected);

    // when we search for projects other than Rust
    let results =
      fetch_artifacts(&temp.to_path_buf(), &custom_project("**/bla"));
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
      fetch_artifacts(&temp.to_path_buf(), &only_rust_projects());

    // then
    assert_eq!(results.len(), 3, "Expected exactly three artifacts");
    results.sort();

    let expected_path_1 =
      temp.child("project1").child("target").path().to_path_buf();
    let expected_1 = ArtifactCandidate::builder(expected_path_1)
      .art_type(Some(ArtifactType::Rust))
      .build();
    assert_eq!(&results[0], &expected_1);

    let expected_path_2 =
      temp.child("project2").child("target").path().to_path_buf();
    let expected_2 = ArtifactCandidate::builder(expected_path_2)
      .art_type(Some(ArtifactType::Rust))
      .build();
    assert_eq!(&results[1], &expected_2);

    let expected_path_3 = temp
      .child("work")
      .child("project3")
      .child("target")
      .path()
      .to_path_buf();
    let expected_3 = ArtifactCandidate::builder(expected_path_3)
      .art_type(Some(ArtifactType::Rust))
      .build();
    assert_eq!(&results[2], &expected_3);

    // when we search for projects other than Rust
    let results =
      fetch_artifacts(&temp.to_path_buf(), &custom_project("**/bla"));
    // then
    assert_eq!(results.len(), 0, "Expected zero artifacts");

    temp.close().unwrap();
  }

  #[test]
  fn test_custom_artifact_type_equivalent_to_rust() {
    // given there is a single rust project in the folder
    let temp = TempDir::new().unwrap();
    mk_rust_project(&temp);

    // when we search for Rust artifacts using built-in type
    let rust_results =
      fetch_artifacts(&temp.to_path_buf(), &only_rust_projects());

    // when we search using Custom type with same pattern and files as Rust
    let custom_rust_type = ArtifactType::Custom {
      pattern: "**/target",
      name: "CustomRust",
      files: &["Cargo.toml"],
    };
    let custom_results =
      fetch_artifacts(&temp.to_path_buf(), &vec![custom_rust_type]);

    // then both should find the same artifact
    assert_eq!(rust_results.len(), 1);
    assert_eq!(custom_results.len(), 1);
    assert_eq!(rust_results[0].path, custom_results[0].path);

    // but the artifact types should be different
    assert_eq!(rust_results[0].art_type, Some(ArtifactType::Rust));
    assert_eq!(
      custom_results[0]
        .art_type
        .as_ref()
        .unwrap()
        .artifact_type_name(),
      "CustomRust"
    );

    temp.close().unwrap();
  }
}
