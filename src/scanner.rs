use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use crate::config::is_path_excluded;
use crate::models::ArtifactItem;
use indicatif::{ProgressBar, ProgressStyle};

/// Target directory names to detect across web, python, rust, C/C++, iOS/Mac, Java/Gradle, and Go ecosystems.
pub const TARGET_PATTERNS: &[&str] = &[
    // Web / JS / TS ecosystem
    "node_modules",
    ".next",
    "dist",
    "build",
    // Python ecosystem
    "__pycache__",
    ".venv",
    // Rust / Cargo ecosystem
    "target",
    // C / C++ / CMake ecosystem
    "cmake-build-debug",
    "cmake-build-release",
    "CMakeFiles",
    // iOS / macOS / Xcode & CocoaPods ecosystem
    "DerivedData",
    "Pods",
    // Java / Kotlin / Gradle ecosystem
    ".gradle",
    // Go ecosystem
    "vendor",
];

/// Checks if the parent directory contains any file with matching extension (e.g. .csproj or .sln).
pub fn has_sibling_file_with_extensions(path: &Path, extensions: &[&str]) -> bool {
    let parent = match path.parent() {
        Some(p) => p,
        None => return false,
    };

    let entries = match fs::read_dir(parent) {
        Ok(e) => e,
        Err(_) => return false,
    };

    for entry in entries.flatten() {
        let entry_path = entry.path();
        if entry_path.is_file() {
            if let Some(ext) = entry_path.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();
                if extensions.iter().any(|&target_ext| target_ext.eq_ignore_ascii_case(&ext_lower)) {
                    return true;
                }
            }
        }
    }

    false
}

/// Checks if the parent directory contains all specified subdirectories (e.g. Assets AND ProjectSettings).
pub fn has_sibling_directories(path: &Path, dir_names: &[&str]) -> bool {
    let parent = match path.parent() {
        Some(p) => p,
        None => return false,
    };

    dir_names.iter().all(|&name| parent.join(name).is_dir())
}

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

/// Recursively scans directory for cleanable artifact pattern matches (simple & context-aware),
/// displays a real-time progress indicator, filters out excluded paths, and sorts results descending by size.
pub fn scan_directory(root: &Path, exclude_patterns: &[String]) -> Vec<ArtifactItem> {
    let mut results = Vec::new();
    let mut it = WalkDir::new(root).into_iter();

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} {msg}")
            .unwrap_or_else(|_| ProgressStyle::default_spinner()),
    );
    pb.set_message("Scanning directory for cleanable artifacts...");
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    let mut scanned_count = 0u64;

    while let Some(entry_res) = it.next() {
        scanned_count += 1;

        let entry = match entry_res {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_dir() {
            continue;
        }

        let file_name = entry.file_name().to_string_lossy();
        let item_path = entry.path();

        let is_simple_match = TARGET_PATTERNS.contains(&file_name.as_ref());

        let is_dotnet_match = (file_name == "bin" || file_name == "obj")
            && has_sibling_file_with_extensions(item_path, &["csproj", "sln"]);

        let is_unity_match = file_name == "Library"
            && has_sibling_directories(item_path, &["Assets", "ProjectSettings"]);

        if is_simple_match || is_dotnet_match || is_unity_match {
            if is_path_excluded(item_path, exclude_patterns) {
                it.skip_current_dir();
                continue;
            }

            let path_buf = item_path.to_path_buf();
            let size = get_dir_size(&path_buf);

            results.push(ArtifactItem {
                name: file_name.to_string(),
                path: path_buf,
                size,
            });

            pb.set_message(format!(
                "Scanning... {} folders checked, {} artifacts found",
                scanned_count,
                results.len()
            ));

            it.skip_current_dir();
        }
    }

    pb.finish_and_clear();

    results.sort_by(|a, b| b.size.cmp(&a.size));
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_scan_directory_v1_and_v2_simple_patterns() {
        let temp_dir = std::env::temp_dir().join("devener_test_v2_simple");
        
        let subfolders = vec![
            temp_dir.join("web/node_modules"),
            temp_dir.join("cpp/cmake-build-debug"),
            temp_dir.join("cpp/CMakeFiles"),
            temp_dir.join("ios/DerivedData"),
            temp_dir.join("ios/Pods"),
            temp_dir.join("android/.gradle"),
            temp_dir.join("go_app/vendor"),
        ];

        for folder in &subfolders {
            fs::create_dir_all(folder).unwrap();
            fs::write(folder.join("dummy.bin"), vec![0u8; 512]).unwrap();
        }

        let items = scan_directory(&temp_dir, &[]);

        assert_eq!(items.len(), 7);
        let names: Vec<String> = items.into_iter().map(|i| i.name).collect();

        assert!(names.contains(&"node_modules".to_string()));
        assert!(names.contains(&"cmake-build-debug".to_string()));
        assert!(names.contains(&"CMakeFiles".to_string()));
        assert!(names.contains(&"DerivedData".to_string()));
        assert!(names.contains(&"Pods".to_string()));
        assert!(names.contains(&".gradle".to_string()));
        assert!(names.contains(&"vendor".to_string()));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_dotnet_context_aware_detection() {
        let temp_dir = std::env::temp_dir().join("devener_test_dotnet");
        
        let valid_project = temp_dir.join("ValidNetApp");
        fs::create_dir_all(valid_project.join("bin")).unwrap();
        fs::create_dir_all(valid_project.join("obj")).unwrap();
        fs::write(valid_project.join("ValidNetApp.csproj"), "<Project></Project>").unwrap();
        fs::write(valid_project.join("bin/app.dll"), vec![0u8; 100]).unwrap();
        fs::write(valid_project.join("obj/app.cache"), vec![0u8; 50]).unwrap();

        let arbitrary_folder = temp_dir.join("RandomFolder");
        fs::create_dir_all(arbitrary_folder.join("bin")).unwrap();
        fs::create_dir_all(arbitrary_folder.join("obj")).unwrap();
        fs::write(arbitrary_folder.join("bin/executable"), vec![0u8; 200]).unwrap();
        fs::write(arbitrary_folder.join("obj/data"), vec![0u8; 100]).unwrap();

        let items = scan_directory(&temp_dir, &[]);

        assert_eq!(items.len(), 2);
        
        let detected_paths: Vec<String> = items.iter().map(|i| i.path.display().to_string()).collect();
        assert!(detected_paths.iter().any(|p| p.contains("ValidNetApp") && p.contains("bin")));
        assert!(detected_paths.iter().any(|p| p.contains("ValidNetApp") && p.contains("obj")));
        assert!(!detected_paths.iter().any(|p| p.contains("RandomFolder")));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_unity_context_aware_detection() {
        let temp_dir = std::env::temp_dir().join("devener_test_unity");

        let valid_unity = temp_dir.join("ValidUnityGame");
        fs::create_dir_all(valid_unity.join("Library")).unwrap();
        fs::create_dir_all(valid_unity.join("Assets")).unwrap();
        fs::create_dir_all(valid_unity.join("ProjectSettings")).unwrap();
        fs::write(valid_unity.join("Library/metadata.cache"), vec![0u8; 300]).unwrap();

        let system_library = temp_dir.join("SystemLibrary");
        fs::create_dir_all(system_library.join("Library")).unwrap();
        fs::write(system_library.join("Library/framework.dll"), vec![0u8; 500]).unwrap();

        let items = scan_directory(&temp_dir, &[]);

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Library");
        assert!(items[0].path.display().to_string().contains("ValidUnityGame"));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
