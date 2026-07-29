use std::path::Path;
use walkdir::WalkDir;
use crate::models::ArtifactItem;

/// MVP target directory names to detect across web, python, and rust ecosystems.
pub const TARGET_PATTERNS: &[&str] = &[
    "node_modules",
    ".next",
    "dist",
    "build",
    "__pycache__",
    ".venv",
    "target",
];

/// Calculates the recursive total size of a directory in bytes.
pub fn get_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|meta| meta.is_file())
        .map(|meta| meta.len())
        .sum()
}

/// Recursively scans directory for cleanable artifact pattern matches,
/// calculates directory sizes, and sorts results descending by size.
pub fn scan_directory(root: &Path) -> Vec<ArtifactItem> {
    let mut results = Vec::new();
    let mut it = WalkDir::new(root).into_iter();

    while let Some(entry_res) = it.next() {
        let entry = match entry_res {
            Ok(e) => e,
            Err(_) => continue,
        };

        let file_name = entry.file_name().to_string_lossy();

        if entry.file_type().is_dir() && TARGET_PATTERNS.contains(&file_name.as_ref()) {
            let item_path = entry.path().to_path_buf();
            let size = get_dir_size(&item_path);

            results.push(ArtifactItem {
                name: file_name.to_string(),
                path: item_path,
                size,
            });

            it.skip_current_dir();
        }
    }

    results.sort_by(|a, b| b.size.cmp(&a.size));
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_scan_directory_and_calculate_size() {
        let test_dir = std::env::temp_dir().join("devener_test_size");
        let node_modules = test_dir.join("app/node_modules");
        fs::create_dir_all(&node_modules).unwrap();

        let dummy_file = node_modules.join("file.bin");
        fs::write(&dummy_file, vec![0u8; 1024]).unwrap();

        let items = scan_directory(&test_dir);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "node_modules");
        assert_eq!(items[0].size, 1024);

        let _ = fs::remove_dir_all(&test_dir);
    }
}
