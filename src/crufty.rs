#![allow(dead_code)]
use std::path::PathBuf;

pub mod cli;

pub struct ArtifactCandidate {
    path: PathBuf,
    size: Option<u64>,
}

pub fn fetch_artifacts(_path : &PathBuf) ->  Vec<ArtifactCandidate> {
  vec![]
}
