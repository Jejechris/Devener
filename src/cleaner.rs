use crate::models::ArtifactItem;
use colored::Colorize;
use humansize::{format_size, DECIMAL};
use inquire::{Confirm, MultiSelect};
use std::fmt;

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

pub fn select_artifacts_to_clean(items: &[ArtifactItem]) -> Vec<ArtifactItem> {
    if items.is_empty() {
        return Vec::new();
    }

    let prompt_msg = "Pilih folder yang ingin dibersihkan (Gunakan SPACE untuk memilih, ENTER untuk konfirmasi):";
    let default_indices: Vec<usize> = (0..items.len()).collect();

    let ans = MultiSelect::new(prompt_msg, items.to_vec())
        .with_default(&default_indices)
        .prompt();

    match ans {
        Ok(selected) => selected,
        Err(_) => Vec::new(),
    }
}

pub fn confirm_deletion(selected_count: usize, total_bytes: u64) -> bool {
    let human_size = format_size(total_bytes, DECIMAL);
    let msg = format!(
        "Anda akan memindahkan {} folder ke Recycle Bin, total {} — lanjutkan?",
        selected_count.to_string().bold().red(),
        human_size.bold().yellow()
    );

    Confirm::new(&msg)
        .with_default(false)
        .prompt()
        .unwrap_or(false)
}

pub fn execute_clean(items: &[ArtifactItem]) -> DeleteReport {
    let mut success_count = 0;
    let mut reclaimed_bytes = 0;
    let mut failed_items = Vec::new();

    println!("\n{}", "Memindahkan folder terpilih ke Recycle Bin...".bold().cyan());

    for item in items {
        match trash::delete(&item.path) {
            Ok(_) => {
                success_count += 1;
                reclaimed_bytes += item.size;
                println!(
                    "  {} {}",
                    "✔".bold().green(),
                    item.path.display()
                );
            }
            Err(err) => {
                let reason = match &err {
                    trash::Error::Os { description, .. } => description.clone(),
                    _ => err.to_string(),
                };
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

    DeleteReport {
        success_count,
        reclaimed_bytes,
        failed_items,
    }
}

pub fn print_final_report(report: &DeleteReport) {
    let human_reclaimed = format_size(report.reclaimed_bytes, DECIMAL);

    println!("\n{}", "=== RINGKASAN PEMBERSIHAN ===".bold().cyan());

    if report.success_count > 0 {
        println!(
            "{} Berhasil memindahkan {} folder ke Recycle Bin",
            "✔".bold().green(),
            report.success_count.to_string().bold().green()
        );
        println!(
            "{} Space yang berhasil direclaim: {}",
            "✔".bold().green(),
            human_reclaimed.bold().yellow()
        );
    }

    for failed in &report.failed_items {
        println!(
            "{} 1 folder gagal dipindahkan ({}): {}",
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
    use std::fs;

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

        let report = execute_clean(&items);

        assert_eq!(report.success_count, 1);
        assert_eq!(report.reclaimed_bytes, 100);
        assert!(!dummy_target.exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
