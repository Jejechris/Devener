mod cleaner;
mod config;
mod models;
mod scanner;

use clap::{Parser, Subcommand};
use colored::Colorize;
use humansize::{format_size, DECIMAL};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "devener")]
#[command(about = "Dev Environment Cleaner CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan target directory for cleanable dev artifacts and caches
    Scan {
        /// Path to scan (defaults to current directory if not specified)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Additional paths or patterns to exclude from scan results
        #[arg(short, long)]
        exclude: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scan { path, exclude } => {
            println!("Scanning directory: {}\n", path.display().to_string().cyan());

            let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            let file_config = config::DevenerConfig::load_from_dir(&current_dir);

            let mut merged_excludes = file_config.exclude;
            merged_excludes.extend(exclude.clone());

            let items = scanner::scan_directory(path, &merged_excludes);

            if items.is_empty() {
                println!("{}", "No cleanable artifacts found.".yellow());
                return;
            }

            let total_bytes: u64 = items.iter().map(|item| item.size).sum();

            println!(
                "Ditemukan {} folder yang bisa dibersihkan:\n",
                items.len().to_string().bold().green()
            );

            for (idx, item) in items.iter().enumerate() {
                let human_size = format_size(item.size, DECIMAL);
                println!(
                    "  [{}] {:<15} {:<45} {}",
                    (idx + 1).to_string().cyan(),
                    item.name.bold(),
                    item.path.display(),
                    human_size.yellow()
                );
            }

            println!(
                "\nTotal potensi space yang bisa direclaim: {}\n",
                format_size(total_bytes, DECIMAL).bold().green()
            );

            let selected = cleaner::select_artifacts_to_clean(&items);

            if selected.is_empty() {
                println!("{}", "Pembersihan dibatalkan. Tidak ada folder yang dipilih.".yellow());
                return;
            }

            let selected_bytes: u64 = selected.iter().map(|item| item.size).sum();

            if cleaner::confirm_deletion(selected.len(), selected_bytes) {
                let report = cleaner::execute_clean(&selected);
                cleaner::print_final_report(&report);
            } else {
                println!("{}", "Pembersihan dibatalkan oleh pengguna.".yellow());
            }
        }
    }
}
