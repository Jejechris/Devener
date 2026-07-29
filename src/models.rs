use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a discovered dev artifact, cache directory, or cleanable file with size in bytes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactItem {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
}

/// Historical cleanup operation record stored in ~/.devener/history.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRecord {
    pub timestamp: String,
    pub items_count: usize,
    pub reclaimed_bytes: u64,
    pub mode: String, // "manual", "auto", "permanent"
    pub target_path: String,
}
