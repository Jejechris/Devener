# devener 🧹 (v5)

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![GitHub release](https://img.shields.io/github/v/release/Jejechris/Devener)](https://github.com/Jejechris/Devener/releases)

**Dev Environment Cleaner CLI** — A fast, cross-platform CLI tool built in Rust to discover, scan, and clean unused build artifacts, cache folders, and academic build outputs across your projects while keeping your files safe.

---

## ✨ Features & Capabilities (v5)

- **🔄 Self-Update Engine (`devener update`) (v5)**: Directly upgrades the executable from the latest GitHub Release binary assets (`devener-windows-amd64.exe`, `devener-linux-amd64`, `devener-macos-amd64`) using `self_update`.
- **💡 Silent Release Check & Notifications (Model B) (v5)**: Non-blocking background thread checks for newer GitHub releases during `devener scan` and displays a polite terminal notification footer if an upgrade is available.
- **🚀 Automated CI/CD Release Pipeline (`.github/workflows/release.yml`) (v5)**: Pushing git release tags (e.g. `v0.5.2`) automatically builds zero-dependency release binaries for Windows, Linux, and macOS, attaching them to GitHub Releases.
- **🌐 English Localization (v4)**: Complete international English user interface, CLI options, warnings, error messages, and summary reports.
- **🛡️ Recycle Bin / Trash Safety Net (Default)**: Items are moved to **Recycle Bin (Windows)** / **Trash (macOS)** / **XDG Trash (Linux)** by default. Accidental deletions can easily be restored from your OS Recycle Bin.
- **⚡ Permanent Deletion Flag (`--permanent`)**: Bypasses the Recycle Bin for direct permanent file/folder removal (`std::fs::remove_dir_all`) with explicit red warning confirmation prompts.
- **🤖 Guarded Automation (`--auto`)**: Run unattended cleanups with strict safety guards:
  - **Requirement 1**: Requires `--older-than <threshold>` (e.g. `--older-than 30d`). Cannot be run without an age filter.
  - **Requirement 2**: Cannot be combined with `--permanent`. Auto mode **ALWAYS** routes deletions through the Recycle Bin.
- **📊 JSON Output Mode (`--json`)**: Output scan results as formatted JSON array to stdout for scripting and CI/CD automation without interactive TTY prompts.
- **⏳ Age Threshold Filter (`--older-than <N>d|h|m`)**: Only target items whose last modification date is older than specified threshold duration (e.g. `30d` for 30 days, `12h` for 12 hours).
- **🎓 Academic & Jupyter Notebook Patterns**:
  - Jupyter Notebook checkpoints: `.ipynb_checkpoints`
  - LaTeX build output files: `.aux`, `.log`, `.out`, `.synctex.gz`
- **📈 Lifetime Cleanup Statistics (`devener stats`)**: Automatically tracks cleanup operations to `~/.devener/history.json` and displays lifetime space reclaimed and historical logs.
- **Multi-Ecosystem Expansion**:
  - **Web / JS / TS**: `node_modules`, `dist`, `build`, `.next`
  - **Python & Jupyter**: `__pycache__`, `.venv`, `.ipynb_checkpoints`
  - **Academic / LaTeX**: `.aux`, `.log`, `.out`, `.synctex.gz`
  - **Rust / Cargo**: `target`
  - **C / C++ / CMake**: `cmake-build-debug`, `cmake-build-release`, `CMakeFiles`
  - **iOS / macOS / Xcode & CocoaPods**: `DerivedData`, `Pods`
  - **Java / Kotlin / Gradle**: `.gradle`
  - **Go**: `vendor`
  - **.NET / C#** (`bin`, `obj`): Context-aware matching requiring `.csproj` or `.sln` in parent folder.
  - **Unity** (`Library`): Context-aware matching requiring `Assets` and `ProjectSettings` in parent folder.
- **Custom Exclude via `devener.toml` & CLI `--exclude`**: Exclude specific directories or projects seamlessly.

---

## 📖 CLI Usage & Examples

### 1. Self-Update (`devener update`)

```bash
# Upgrade devener executable to latest version from GitHub Releases
devener update
```

### 2. Interactive Scan (Default - Safe to Recycle Bin)

```bash
devener scan
```

### 3. Exclude Specific Paths

```bash
devener scan --exclude ./my-app/node_modules --exclude ./rust/target
```

### 4. Filter by Item Age (`--older-than`)

```bash
# Only scan items older than 30 days
devener scan --older-than 30d
```

### 5. Output as Formatted JSON (`--json`)

```bash
devener scan --json
```

### 6. Guarded Auto-Clean (`--auto`)

```bash
devener scan --auto --older-than 30d
```

### 7. Permanent Deletion (`--permanent`)

```bash
devener scan --permanent
```

### 8. View Lifetime Stats (`devener stats`)

```bash
devener stats
```

---

## 🛠️ Development & Building

```bash
# Run full unit test suite
cargo test

# Build optimized release binary
cargo build --release

# Install locally to Cargo path
cargo install --path .
```

---

## 📜 License

GNU General Public License v3.0 (GPLv3) © 2026 Jejechris
