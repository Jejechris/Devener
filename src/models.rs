use std::path::PathBuf;

/// Represents a discovered dev artifact or cache directory with size in bytes.
#[derive(Debug, Clone)]
pub struct ArtifactItem {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
}
