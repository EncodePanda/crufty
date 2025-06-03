#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ArtifactType {
  Rust,
  Scala,
  JavaMaven,
  Custom {
    pattern: &'static str,
    name: &'static str,
    files: &'static [&'static str],
  },
}

pub fn builtin() -> [ArtifactType; 3] {
  [
    ArtifactType::Rust,
    ArtifactType::Scala,
    ArtifactType::JavaMaven,
  ]
}

impl ArtifactType {
  pub fn pattern(&self) -> &'static str {
    match self {
      ArtifactType::Rust => "**/target",
      ArtifactType::Scala => "**/target",
      ArtifactType::JavaMaven => "**/target",
      ArtifactType::Custom { pattern, .. } => pattern,
    }
  }

  pub fn artifact_type_name(&self) -> &'static str {
    match self {
      ArtifactType::Rust => "Rust",
      ArtifactType::Scala => "Scala",
      ArtifactType::JavaMaven => "Java",
      ArtifactType::Custom { name, .. } => name,
    }
  }

  pub fn recognized_files(&self) -> Vec<String> {
    match self {
      ArtifactType::Rust => vec!["Cargo.toml".to_string()],
      ArtifactType::Scala => vec!["build.sbt".to_string()],
      ArtifactType::JavaMaven => vec!["pom.xml".to_string()],
      ArtifactType::Custom { files, .. } => {
        files.iter().map(|s| s.to_string()).collect()
      }
    }
  }
}
