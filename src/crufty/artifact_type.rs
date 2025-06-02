#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ArtifactType {
  Rust,
  Scala,
  Custom { pattern: &'static str },
}

pub fn builtin() -> [ArtifactType; 2] {
  [ArtifactType::Rust, ArtifactType::Scala]
}

impl ArtifactType {
  pub fn pattern(&self) -> &'static str {
    match self {
      ArtifactType::Rust => "**/target",
      ArtifactType::Scala => "**/target",
      ArtifactType::Custom { pattern } => pattern,
    }
  }
}
