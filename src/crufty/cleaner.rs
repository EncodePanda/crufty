#![allow(dead_code)]
use fs_extra::dir::remove;

use super::types::{ArtifactCandidate, Size};

pub enum CleanupResult {
  Successful { size: Size },
  DoesNotExists,
  Unsuccessful,
}

pub fn clean(artifact: &ArtifactCandidate) -> CleanupResult {
  let path = &artifact.path;

  match path.exists() {
    false => CleanupResult::DoesNotExists,
    true => match remove(path) {
      Ok(_) => CleanupResult::Successful {
        size: artifact.size.clone(),
      },
      Err(_) => CleanupResult::Unsuccessful,
    },
  }
}
