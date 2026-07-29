use colored::Colorize;
use self_update::backends::github::Update;

pub const REPO_OWNER: &str = "Jejechris";
pub const REPO_NAME: &str = "Devener";

/// Executes self-update by checking GitHub Releases and replacing the current executable.
pub fn run_update() {
    let current_version = env!("CARGO_PKG_VERSION");
    println!(
        "Checking for updates on GitHub (current version: v{})...",
        current_version.cyan()
    );

    let target_bin_name = if cfg!(target_os = "windows") {
        "devener-windows-amd64.exe"
    } else if cfg!(target_os = "macos") {
        "devener-macos-amd64"
    } else {
        "devener-linux-amd64"
    };

    let status = Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name(target_bin_name)
        .show_download_progress(true)
        .current_version(current_version)
        .build();

    let status = match status {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Failed to configure self-update: {}", "✖".bold().red(), e);
            return;
        }
    };

    match status.update() {
        Ok(update_status) => {
            if update_status.updated() {
                println!(
                    "\n{} Successfully updated devener to version v{}!",
                    "✔".bold().green(),
                    update_status.version().bold().green()
                );
            } else {
                println!(
                    "{} You are already on the latest version of devener (v{}).",
                    "✔".bold().green(),
                    current_version.bold().green()
                );
            }
        }
        Err(e) => {
            eprintln!(
                "{} Update check finished: {} (No newer release binary found on GitHub)",
                "ℹ".bold().cyan(),
                e
            );
        }
    }
}

/// Checks silently if a newer release tag is available on GitHub (Model B notification check).
pub fn check_for_latest_release_silent() -> Option<String> {
    let current_version = env!("CARGO_PKG_VERSION");

    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .build()
        .ok()?
        .fetch()
        .ok()?;

    let latest_release = releases.first()?;
    let latest_version_str = latest_release.version.trim_start_matches('v');

    if self_update::version::bump_is_greater(current_version, latest_version_str).unwrap_or(false) {
        Some(latest_release.version.clone())
    } else {
        None
    }
}
