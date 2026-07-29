mod cleaner;
mod config;
mod models;
mod scanner;
mod stats;
mod update;

use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;
use std::process;
use std::thread;

#[derive(Parser)]
#[command(name = "devener")]
#[command(about = "Dev Environment Cleaner CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan target directory for cleanable dev artifacts, caches, and build output
    Scan {
        /// Path to scan (defaults to current directory if not specified)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Additional paths or patterns to exclude from scan results
        #[arg(short, long)]
        exclude: Vec<String>,

        /// Permanently delete items instead of moving to OS Recycle Bin
        #[arg(long)]
        permanent: bool,

        /// Output scan results as formatted JSON (skips interactive mode)
        #[arg(long)]
        json: bool,

        /// Filter items modified older than threshold (e.g. 30d, 12h, 60m)
        #[arg(long = "older-than")]
        older_than: Option<String>,

        /// Auto-clean matching items without interactive prompts (requires --older-than)
        #[arg(long)]
        auto: bool,
    },
    /// Show lifetime cleanup statistics and operation history log
    Stats,
    /// Update devener executable to the latest version from GitHub Releases
    Update,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Update => {
            update::run_update();
        }
        Commands::Stats => {
            stats::print_stats();
        }
        Commands::Scan {
            path,
            exclude,
            permanent,
            json,
            older_than,
            auto,
        } => {
            // Model B: Spawn background thread to check for updates silently if not in JSON mode
            let update_handle = if !*json {
                Some(thread::spawn(update::check_for_latest_release_silent))
            } else {
                None
            };

            // Rule 1: --auto requires --older-than
            if *auto && older_than.is_none() {
                eprintln!(
                    "{}",
                    "Error: Safety requirement violated. --auto mode requires --older-than <threshold> (e.g. --older-than 30d)."
                        .bold()
                        .red()
                );
                process::exit(1);
            }

            // Rule 2: --auto cannot be combined with --permanent
            if *auto && *permanent {
                eprintln!(
                    "{}",
                    "Error: Safety requirement violated. --auto mode cannot be combined with --permanent. Auto mode ALWAYS uses Recycle Bin."
                        .bold()
                        .red()
                );
                process::exit(1);
            }

            // Parse age threshold if provided
            let age_duration = if let Some(ref s) = older_than {
                match scanner::parse_age_threshold(s) {
                    Some(d) => Some(d),
                    None => {
                        eprintln!(
                            "{}",
                            format!("Error: Invalid age threshold format '{}'. Use e.g. 30d, 12h, 60m.", s)
                                .bold()
                                .red()
                        );
                        process::exit(1);
                    }
                }
            } else {
                None
            };

            let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            let file_config = config::DevenerConfig::load_from_dir(&current_dir);

            let mut merged_excludes = file_config.exclude;
            merged_excludes.extend(exclude.clone());

            let show_progress = !*json;
            let items = scanner::scan_directory(path, &merged_excludes, age_duration, show_progress);

            // Handle --json output mode
            if *json {
                match serde_json::to_string_pretty(&items) {
                    Ok(json_str) => println!("{}", json_str),
                    Err(e) => eprintln!("Error generating JSON: {}", e),
                }
                return;
            }

            println!("Scanning directory: {}\n", path.display().to_string().cyan());

            if items.is_empty() {
                println!("{}", "No cleanable artifacts found.".yellow());
                print_update_notification_if_available(update_handle);
                return;
            }

            let total_bytes: u64 = items.iter().map(|item| item.size).sum();

            println!(
                "Found {} cleanable item(s):\n",
                items.len().to_string().bold().green()
            );

            for (idx, item) in items.iter().enumerate() {
                let human_size = humansize::format_size(item.size, humansize::DECIMAL);
                println!(
                    "  [{}] {:<15} {:<45} {}",
                    (idx + 1).to_string().cyan(),
                    item.name.bold(),
                    item.path.display(),
                    human_size.yellow()
                );
            }

            println!(
                "\nTotal potential space reclaimable: {}\n",
                humansize::format_size(total_bytes, humansize::DECIMAL).bold().green()
            );

            // Handle --auto mode
            if *auto {
                println!("{}", "Auto mode active. Processing items automatically...".cyan());
                let report = cleaner::execute_clean(&items, false, "auto", &path.display().to_string());
                cleaner::print_final_report(&report, false);
                print_update_notification_if_available(update_handle);
                return;
            }

            // Interactive mode
            let selected = cleaner::select_artifacts_to_clean(&items);

            if selected.is_empty() {
                println!("{}", "Cleanup cancelled. No items selected.".yellow());
                print_update_notification_if_available(update_handle);
                return;
            }

            let selected_bytes: u64 = selected.iter().map(|item| item.size).sum();

            let confirmed = if *permanent {
                cleaner::confirm_permanent_deletion(selected.len(), selected_bytes)
            } else {
                cleaner::confirm_deletion(selected.len(), selected_bytes)
            };

            if confirmed {
                let mode_str = if *permanent { "permanent" } else { "manual" };
                let report = cleaner::execute_clean(&selected, *permanent, mode_str, &path.display().to_string());
                cleaner::print_final_report(&report, *permanent);
            } else {
                println!("{}", "Cleanup cancelled by user.".yellow());
            }

            print_update_notification_if_available(update_handle);
        }
    }
}

/// Helper function to join the background update check thread and print notification footer if available.
fn print_update_notification_if_available(handle: Option<thread::JoinHandle<Option<String>>>) {
    if let Some(h) = handle {
        if let Ok(Some(latest_ver)) = h.join() {
            println!(
                "\n{} A new version of devener is available ({})! Run '{}' to upgrade.",
                "💡".yellow(),
                latest_ver.bold().green(),
                "devener update".cyan()
            );
        }
    }
}
