# devener 🧹 (v4)

**Dev Environment Cleaner CLI** — A fast, cross-platform CLI tool built in Rust to discover, scan, and clean unused build artifacts, cache folders, and academic build outputs across your projects while keeping your files safe.

---

## ✨ Features & Capabilities (v4)

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

## ⚙️ Configuration File (`devener.toml`)

Create a `devener.toml` file in your working directory to exclude specific folders:

```toml
# Example devener.toml
exclude = [
    "./important-project/node_modules",
    "./keep-this/target"
]
```

---

## 📖 CLI Usage & Examples

### 1. Interactive Scan (Default - Safe to Recycle Bin)

```bash
devener scan
```

### 2. Exclude Specific Paths

```bash
devener scan --exclude ./my-app/node_modules --exclude ./rust/target
```

### 3. Filter by Item Age (`--older-than`)

```bash
# Only scan items older than 30 days
devener scan --older-than 30d

# Only scan items older than 12 hours
devener scan --older-than 12h
```

### 4. Output as Formatted JSON (`--json`)

```bash
devener scan --json
```

Output:
```json
[
  {
    "name": "node_modules",
    "path": "./web_app/node_modules",
    "size": 1240000000
  }
]
```

### 5. Guarded Auto-Clean (`--auto`)

```bash
# Safe automated cleanup of items older than 30 days (moves to Recycle Bin)
devener scan --auto --older-than 30d
```

### 6. Permanent Deletion (`--permanent`)

```bash
# Bypasses Recycle Bin with explicit confirmation warning
devener scan --permanent
```

### 7. View Lifetime Stats (`devener stats`)

```bash
devener stats
```

---

## 🛠️ Development & Building

```bash
# Run full unit test suite (7 tests)
cargo test

# Build optimized release binary
cargo build --release

# Install locally to Cargo path
cargo install --path .
```

---

## 📜 License

MIT License © 2026
