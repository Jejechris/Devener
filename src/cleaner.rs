use crate::models::ArtifactItem;
use crate::stats;
use colored::Colorize;
use humansize::{format_size, DECIMAL};
use inquire::{Confirm, MultiSelect};
use std::fmt;
use std::fs;

impl fmt::Display for ArtifactItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let human_size = format_size(self.size, DECIMAL);
        write!(f, "{:<12} {:<40} {}", self.name, self.path.display(), human_size)
    }
}

pub struct FailedItem {
    pub path: String,
    pub reason: String,
}

pub struct DeleteReport {
    pub success_count: usize,
    pub reclaimed_bytes: u64,
    pub failed_items: Vec<FailedItem>,
}

/// Prompts user to interactively select which artifact folders to clean.
pub fn select_artifacts_to_clean(items: &[ArtifactItem]) -> Vec<ArtifactItem> {
    if items.is_empty() {
        return Vec::new();
    }

    let prompt_msg = "Select artifact folders to clean (Use SPACE to select/deselect, ENTER to confirm):";
    let default_indices: Vec<usize> = (0..items.len()).collect();

    let ans = MultiSelect::new(prompt_msg, items.to_vec())
        .with_default(&default_indices)
        .prompt();

    match ans {
        Ok(selected) => selected,
        Err(_) => Vec::new(),
    }
}

/// Prompts final confirmation before moving items to Recycle Bin.
pub fn confirm_deletion(selected_count: usize, total_bytes: u64) -> bool {
    let human_size = format_size(total_bytes, DECIMAL);
    let msg = format!(
        "You are about to move {} folder(s) to Recycle Bin, total {} — proceed?",
        selected_count.to_string().bold().red(),
        human_size.bold().yellow()
    );

    Confirm::new(&msg)
        .with_default(false)
        .prompt()
        .unwrap_or(false)
}

/// Prompts explicit warning & confirmation for permanent deletion.
pub fn confirm_permanent_deletion(selected_count: usize, total_bytes: u64) -> bool {
    let human_size = format_size(total_bytes, DECIMAL);

    println!(
        "\n{}",
        "⚠️  WARNING: --permanent flag detected. Selected items will be PERMANENTLY deleted and CANNOT be restored from Recycle Bin."
            .bold()
            .red()
    );

    let msg = format!(
        "Are you absolutely sure you want to PERMANENTLY delete {} item(s), total {}?",
        selected_count.to_string().bold().red(),
        human_size.bold().yellow()
    );

    Confirm::new(&msg)
        .with_default(false)
        .prompt()
        .unwrap_or(false)
}

/// Executes cleanup (trash-based or permanent) and saves stats history.
pub fn execute_clean(
    items: &[ArtifactItem],
    is_permanent: bool,
    mode_name: &str,
    target_dir_str: &str,
) -> DeleteReport {
    let mut success_count = 0;
    let mut reclaimed_bytes = 0;
    let mut failed_items = Vec::new();

    if is_permanent {
        println!("\n{}", "PERMANENTLY deleting selected item(s)...".bold().red());
    } else {
        println!("\n{}", "Moving selected folder(s) to Recycle Bin...".bold().cyan());
    }

    for item in items {
        let result = if is_permanent {
            if item.path.is_dir() {
                fs::remove_dir_all(&item.path).map_err(|e| e.to_string())
            } else {
                fs::remove_file(&item.path).map_err(|e| e.to_string())
            }
        } else {
            trash::delete(&item.path).map_err(|err| match &err {
                trash::Error::Os { description, .. } => description.clone(),
                _ => err.to_string(),
            })
        };

        match result {
            Ok(_) => {
                success_count += 1;
                reclaimed_bytes += item.size;
                println!(
                    "  {} {}",
                    "✔".bold().green(),
                    item.path.display()
                );
            }
            Err(reason) => {
                println!(
                    "  {} {} ({})",
                    "✖".bold().red(),
                    item.path.display(),
                    reason.red()
                );
                failed_items.push(FailedItem {
                    path: item.path.display().to_string(),
                    reason,
                });
            }
        }
    }

    if success_count > 0 {
        stats::save_history(success_count, reclaimed_bytes, mode_name, target_dir_str);
    }

    DeleteReport {
        success_count,
        reclaimed_bytes,
        failed_items,
    }
}

/// Displays final summary report after cleanup.
pub fn print_final_report(report: &DeleteReport, is_permanent: bool) {
    let human_reclaimed = format_size(report.reclaimed_bytes, DECIMAL);

    println!("\n{}", "=== CLEANUP SUMMARY ===".bold().cyan());

    if report.success_count > 0 {
        if is_permanent {
            println!(
                "{} Successfully PERMANENTLY deleted {} item(s)",
                "✔".bold().green(),
                report.success_count.to_string().bold().green()
            );
        } else {
            println!(
                "{} Successfully moved {} item(s) to Recycle Bin",
                "✔".bold().green(),
                report.success_count.to_string().bold().green()
            );
        }
        println!(
            "{} Space reclaimed: {}",
            "✔".bold().green(),
            human_reclaimed.bold().yellow()
        );
    }

    for failed in &report.failed_items {
        println!(
            "{} 1 item failed to process ({}): {}",
            "✖".bold().red(),
            failed.reason,
            failed.path
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ArtifactItem;

    #[test]
    fn test_execute_clean_deletes_directory() {
        let temp_dir = std::env::temp_dir().join("devener_test_trash");
        let dummy_target = temp_dir.join("dummy_node_modules");
        fs::create_dir_all(&dummy_target).unwrap();
        fs::write(dummy_target.join("package.json"), "{}").unwrap();

        let items = vec![ArtifactItem {
            name: "dummy_node_modules".to_string(),
            path: dummy_target.clone(),
            size: 100,
        }];

        let report = execute_clean(&items, false, "test", "test_path");

        assert_eq!(report.success_count, 1);
        assert_eq!(report.reclaimed_bytes, 100);
        assert!(!dummy_target.exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
