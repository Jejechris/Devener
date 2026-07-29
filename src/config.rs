use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Configuration struct mapped from `devener.toml`.
#[derive(Debug, Deserialize, Default)]
pub struct DevenerConfig {
    #[serde(default)]
    pub exclude: Vec<String>,
}

impl DevenerConfig {
    /// Loads `devener.toml` from working directory if present.
    pub fn load_from_dir(dir: &Path) -> Self {
        let config_file = dir.join("devener.toml");
        if !config_file.is_file() {
            return Self::default();
        }

        match fs::read_to_string(&config_file) {
            Ok(content) => match toml::from_str(&content) {
                Ok(cfg) => cfg,
                Err(err) => {
                    eprintln!("Warning: Failed to parse devener.toml: {}", err);
                    Self::default()
                }
            },
            Err(_) => Self::default(),
        }
    }
}

/// Checks if an item path matches any excluded pattern string.
pub fn is_path_excluded(item_path: &Path, exclude_patterns: &[String]) -> bool {
    if exclude_patterns.is_empty() {
        return false;
    }

    let item_str = item_path.to_string_lossy();
    let norm_item = item_str.replace('\\', "/");

    for pattern in exclude_patterns {
        let norm_pattern = pattern.replace('\\', "/");
        let trimmed = norm_pattern.trim_start_matches("./");

        if norm_item == norm_pattern
            || norm_item.ends_with(trimmed)
            || norm_item.contains(trimmed)
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_is_path_excluded_matching() {
        let excludes = vec![
            "./important-project/node_modules".to_string(),
            "keep-this/target".to_string(),
        ];

        let p1 = PathBuf::from("projects/important-project/node_modules");
        let p2 = PathBuf::from("projects/other-project/node_modules");
        let p3 = PathBuf::from("work\\keep-this\\target");

        assert!(is_path_excluded(&p1, &excludes));
        assert!(!is_path_excluded(&p2, &excludes));
        assert!(is_path_excluded(&p3, &excludes));
    }
}
