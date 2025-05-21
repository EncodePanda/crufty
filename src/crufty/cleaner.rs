#![allow(dead_code)]
use fs_extra::dir::remove;

use super::types::ArtifactCandidate;

pub enum CleanupResult {
  Successful,
  DoesNotExists,
  Unsuccessful,
}

pub fn clean(artifact: &ArtifactCandidate) -> CleanupResult {
  let path = &artifact.path;

  match path.exists() {
    false => CleanupResult::DoesNotExists,
    true => match remove(path) {
      Ok(_) => CleanupResult::Successful,
      Err(_) => CleanupResult::Unsuccessful,
    },
  }
}
