use crate::models::HistoryRecord;
use colored::Colorize;
use humansize::{format_size, DECIMAL};
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

/// Gets cross-platform history file path: ~/.devener/history.json
pub fn get_history_file_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".devener").join("history.json"))
}

/// Appends a new cleanup operation record to ~/.devener/history.json
pub fn save_history(count: usize, reclaimed_bytes: u64, mode: &str, target_path: &str) {
    let history_file = match get_history_file_path() {
        Some(p) => p,
        None => return,
    };

    if let Some(parent) = history_file.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut records: Vec<HistoryRecord> = if history_file.is_file() {
        fs::read_to_string(&history_file)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    let now_str = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => format!("Unix Timestamp {}", d.as_secs()),
        Err(_) => "Unknown".to_string(),
    };

    records.push(HistoryRecord {
        timestamp: now_str,
        items_count: count,
        reclaimed_bytes,
        mode: mode.to_string(),
        target_path: target_path.to_string(),
    });

    if let Ok(json) = serde_json::to_string_pretty(&records) {
        let _ = fs::write(history_file, json);
    }
}

/// Prints lifetime stats summary and recent cleanup operation log.
pub fn print_stats() {
    let history_file = match get_history_file_path() {
        Some(p) => p,
        None => {
            println!("{}", "Could not determine user home directory.".red());
            return;
        }
    };

    if !history_file.is_file() {
        println!("{}", "No cleanup history found yet. Run 'devener scan' to reclaim space!".yellow());
        return;
    }

    let content = match fs::read_to_string(&history_file) {
        Ok(c) => c,
        Err(_) => {
            println!("{}", "Failed to read history log.".red());
            return;
        }
    };

    let records: Vec<HistoryRecord> = match serde_json::from_str(&content) {
        Ok(r) => r,
        Err(_) => {
            println!("{}", "Failed to parse history log.".red());
            return;
        }
    };

    if records.is_empty() {
        println!("{}", "No cleanup history found yet.".yellow());
        return;
    }

    let total_ops = records.len();
    let total_reclaimed: u64 = records.iter().map(|r| r.reclaimed_bytes).sum();
    let total_items: usize = records.iter().map(|r| r.items_count).sum();

    println!("\n{}", "=== DEVENER LIFETIME CLEANUP STATS ===".bold().cyan());
    println!("  Total cleanup operations: {}", total_ops.to_string().bold().green());
    println!("  Total items cleaned:      {}", total_items.to_string().bold().green());
    println!(
        "  Total space reclaimed:    {}\n",
        format_size(total_reclaimed, DECIMAL).bold().yellow()
    );

    println!("{}", "Recent History:".bold().white());
    let recent_count = records.len().min(5);
    for record in records.iter().rev().take(recent_count) {
        let human_size = format_size(record.reclaimed_bytes, DECIMAL);
        println!(
            "  • [{}] Mode: {:<10} Reclaimed: {:<10} Items: {}",
            record.target_path.cyan(),
            record.mode.bold(),
            human_size.yellow(),
            record.items_count
        );
    }
}
