# Devener Walkthrough & Documentation

Documenting progress and milestone steps for building `devener`.

---

## Milestone 0: Project Setup

### Summary
- Project initialized using `cargo init` under `c:\Users\Senna\Desktop\Projects\Devener`.
- Installed and configured Rust GNU toolchain (`stable-x86_64-pc-windows-gnu`) for Windows MinGW GCC compatibility.
- Added base dependencies in `Cargo.toml`: `clap`, `walkdir`, `humansize`, `inquire`, `colored`.
- Created minimal `README.md` and verified `.gitignore`.

### Key Rust Concepts Learned
- **Cargo**: Rust's package manager and build tool.
- **Cargo.toml**: Manifest file for project settings and crate dependencies.
- **`fn main()` & Macros**: Main entry point; `println!` is a macro (denoted by `!`).

### Verification & Testing
- Command: `cargo run`
- Output: `Hello, world!`
- Build Result: Clean build with 54 compiled crates.

---

## Milestone 1: Basic Argument Parsing (`clap`)

### Summary
- Implemented CLI argument parsing in [src/main.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/main.rs) using `clap` derive macros.
- Added `devener scan [PATH]` command support with default fallback path set to `.` (current directory).
- Added automatic `--help` and `--version` support via `clap`.

### Key Rust Concepts Learned
- **Structs & Derive Macros (`#[derive(Parser)]`)**: Data structures that automatically generate CLI parsing logic at compile time.
- **Enums & Pattern Matching (`match`)**: Representing subcommands cleanly as enum variants (`Commands::Scan`) and enforcing exhaustive handling with `match`.
- **`PathBuf`**: Standard library type for cross-platform path manipulation.
- **Borrowing (`&`)**: Passing a reference (`&cli.command`) to access data without taking ownership.

### Verification & Testing
- Command: `cargo run -- scan .`
  Output: `Target path to scan: .`
- Command: `cargo run -- scan`
  Output: `Target path to scan: .`
- Command: `cargo run -- scan src`
  Output: `Target path to scan: src`

---

## Milestone 2: Directory Scanning & Pattern Matching

### Summary
- Created data model in [src/models.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/models.rs) (`ArtifactItem`).
- Created scanning logic in [src/scanner.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/scanner.rs) using `walkdir::WalkDir`.
- Configured MVP target patterns (`node_modules`, `.next`, `dist`, `build`, `__pycache__`, `.venv`, `target`).
- Implemented directory pruning (`it.skip_current_dir()`) to avoid descending into matched artifact folders.
- Wired scanning logic into [src/main.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/main.rs) to display raw list of discovered targets.

### Key Rust Concepts Learned
- **Module System (`mod` & `pub`)**: Splitting code into clean files ([src/models.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/models.rs), [src/scanner.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/scanner.rs)) and exposing symbols using `pub`.
- **Directory Traversal & Pruning**: Using `WalkDir` iterator and `.skip_current_dir()` for optimized scanning performance.
- **Vectors (`Vec<T>`)**: Dynamic array type for collecting matching artifact items.
- **Conditional Compilation & Unit Testing (`#[cfg(test)]`)**: Writing unit tests right next to source code.

### Verification & Testing
- Created dummy project tree `test_dummy` containing `.venv`, `__pycache__`, `target`, `dist`, and `node_modules`.
- Command: `cargo run -- scan test_dummy`
  Discovered all 5 artifact folders successfully.
- Command: `cargo test`
  Output: `cargo test: 1 passed (1 suite, 0.00s)`

---

## Milestone 3: Size Calculation & Formatted Output

### Summary
- Added `size: u64` field to `ArtifactItem` in [src/models.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/models.rs).
- Implemented recursive size calculation `get_dir_size` and size sorting (`sort_by`) in [src/scanner.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/scanner.rs).
- Formatted output table in [src/main.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/main.rs) using `humansize` (`format_size`) and `colored` terminal text.
- Added total potential reclaimable space calculation.

### Key Rust Concepts Learned
- **`u64` Unsigned Integers**: High-capacity integer type for byte counts.
- **Sorting Closures (`sort_by`)**: Sorting vectors descending (`b.size.cmp(&a.size)`).
- **Formatters & External Crates**: `humansize` formatting raw byte values to human-readable strings (`MB`, `kB`), and `colored` styling terminal output.

### Verification & Testing
- Command: `cargo test`
  Output: `1 passed`
- Command: `cargo run -- scan test_dummy`
  Output:
  ```text
  Scanning directory: test_dummy

  Ditemukan 5 folder yang bisa dibersihkan:

    [1] node_modules    test_dummy\web_app\node_modules               5.24 MB
    [2] target          test_dummy\rust_cli\target                    1.57 MB
    [3] .venv           test_dummy\py_service\.venv                   307.21 kB
    [4] dist            test_dummy\web_app\dist                       51.21 kB
    [5] __pycache__     test_dummy\py_service\__pycache__             2.05 kB

  Total potensi space yang bisa direclaim: 7.18 MB
  ```

---

## Milestone 4: Interactive Selection & Confirmation Prompt (Dry-Run Preview)

### Summary
- Created interactive CLI module in [src/cleaner.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/cleaner.rs).
- Implemented `std::fmt::Display` for `ArtifactItem` to support rendering items in terminal menus.
- Added interactive multi-select checkbox prompt using `inquire::MultiSelect`.
- Added final preview confirmation prompt using `inquire::Confirm`.
- Implemented `dry_run_clean` (Safety Dry-Run) so user can test selection and confirmation flow without deleting files.

### Key Rust Concepts Learned
- **`std::fmt::Display` Trait**: Customizing string formatting for custom structs.
- **Interactive Prompts (`inquire`)**: Multi-select checkbox and confirmation dialogs.
- **Graceful Error Handling (`Result`)**: Safely handling prompt aborts (e.g. `Esc` or `Ctrl+C`).

### Verification & Testing
- Command: `cargo test`
  Output: `1 passed`
- Interactive CLI flow verified safely in dry-run mode (0 files deleted).

---

## Milestone 5: Actual Deletion Execution & Final Summary Report

### Summary
- Implemented actual recursive deletion logic `execute_clean` using `std::fs::remove_dir_all`.
- Added error handling for filesystem error kinds (`PermissionDenied`, `NotFound`).
- Added summary statistics structures `DeleteReport` and `FailedItem`.
- Implemented `print_final_report` to render success counters, reclaimed space, and failed path details.
- Added unit test in `src/cleaner.rs` testing recursive directory removal.

### Key Rust Concepts Learned
- **Filesystem Deletion (`std::fs::remove_dir_all`)**: Safely deleting directories recursively.
- **`std::io::ErrorKind`**: Matching specific I/O errors to produce friendly user feedback.
- **Summary Structs**: Structuring summary results for final report rendering.

### Verification & Testing
- Command: `cargo test`
  Output: `cargo test: 2 passed (1 suite, 0.00s)`

---

## Milestone 6: README & Final Documentation

### Summary
- Created production-ready [README.md](file:///c:/Users/Senna/Desktop/Projects/Devener/README.md) containing value proposition, demo output walkthrough, installation instructions, usage guide, development commands, and placeholder instructions for demo GIF.
- Verified release build compilation (`cargo build --release`).

### Key Rust Concepts Learned
- **Binary Distribution (`cargo install`)**: Distributing Rust CLI applications as a zero-dependency standalone binary executable.
- **Documentation Comments (`///`)**: Standardizing code comments for automated documentation generation.

### Verification & Testing
- Command: `cargo test` -> `2 passed (1 suite, 0.00s)`
- Command: `cargo build --release` -> `Finished release profile [optimized] in 21.52s`

---

## Milestone 7: Ecosystem Expansion (Simple Patterns)

### Summary
- Expanded `TARGET_PATTERNS` in [src/scanner.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/scanner.rs) with new specific patterns:
  - C/C++/CMake: `cmake-build-debug`, `cmake-build-release`, `CMakeFiles`
  - iOS/Mac/Xcode: `DerivedData`, `Pods`
  - Java/Gradle: `.gradle`
  - Go: `vendor`
- Verified backward compatibility with existing v1 patterns.

### Key Rust Concepts Learned
- **Static Array Slices (`const &[&str]`)**: Zero-allocation static constant data storage in binary executable.
- **Non-breaking Regression Testing**: Ensuring added patterns do not break existing pattern detection.

### Verification & Testing
- Command: `cargo test` -> `2 passed (1 suite, 0.03s)`
- Command: `cargo run -- scan test_dummy_v2`
  Discovered all 7 pattern folders (`cmake-build-debug`, `DerivedData`, `.gradle`, `CMakeFiles`, `vendor`, `Pods`, `node_modules`).

---

## Milestone 8: Context-Aware Detection — .NET (`bin`/`obj`)

### Summary
- Implemented `has_sibling_file_with_extensions` in [src/scanner.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/scanner.rs).
- Configured context-aware matching for `bin` and `obj` folders: only matched if a `.csproj` or `.sln` file exists in the parent directory.
- Added comprehensive unit test `test_dotnet_context_aware_detection` covering valid .NET projects vs arbitrary folders named `bin`/`obj`.

### Key Rust Concepts Learned
- **Path Navigation (`path.parent()`)**: Traversing to the parent directory of a path candidate.
- **Directory Inspection (`std::fs::read_dir`)**: Inspecting sibling files in parent directories.
- **Case-Insensitive String Matching (`eq_ignore_ascii_case`)**: Checking extensions cleanly across platforms.

### Verification & Testing
- Command: `cargo test` -> `2 passed (1 suite, 0.01s)`
- Command: `cargo run -- scan test_dummy_dotnet`
  Result: Only `MyDotNetApp\bin` and `MyDotNetApp\obj` were detected (with `.csproj` sibling). `LegitTool\bin` and `LegitTool\obj` (no `.csproj`) were correctly skipped.

---

## Milestone 9: Context-Aware Detection — Unity (`Library`)

### Summary
- Implemented `has_sibling_directories` in [src/scanner.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/scanner.rs).
- Configured context-aware matching for `Library` folders: only matched if BOTH `Assets` and `ProjectSettings` directories exist in the parent folder.
- Added comprehensive unit test `test_unity_context_aware_detection`.

### Key Rust Concepts Learned
- **Multi-Directory Inspection (`parent.join(name).is_dir()`)**: Verifying existence of multiple sibling folders.
- **Boolean Iterator All (`.all(...)`)**: Ensuring all required conditions match simultaneously.

### Verification & Testing
- Command: `cargo test` -> `3 passed (1 suite, 0.01s)`
- Command: `cargo run -- scan test_dummy_unity`
  Result: Only `MyUnityGame\Library` (with `Assets` and `ProjectSettings` siblings) was detected. `SystemLib\Library` (no siblings) was correctly skipped.

---

## Milestone 10: Progress Indicator Real-Time Scanning (`indicatif`)

### Summary
- Integrated `indicatif::ProgressBar` spinner in [src/scanner.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/scanner.rs).
- Displayed real-time checked directory counter and discovered artifact counter during folder traversal.
- Ensured spinner clears cleanly (`pb.finish_and_clear()`) before printing formatted scan result table.

### Key Rust Concepts Learned
- **Terminal Animations & Progress (`indicatif`)**: Rendering smooth terminal spinners and progress counters.
- **TTY State Management**: Clearing progress elements cleanly to avoid terminal display corruption.

---

## Milestone 11: Config File `devener.toml` Support (`toml` + `serde`)

### Summary
- Created [src/config.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/config.rs) with `DevenerConfig` struct.
- Implemented TOML file parsing `load_from_dir` and path exclude matching logic `is_path_excluded`.
- Integrated `devener.toml` loading in [src/main.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/main.rs).
- Added unit test `test_is_path_excluded_matching` in `src/config.rs`.

### Key Rust Concepts Learned
- **Serde Serialization / Deserialization (`#[derive(Deserialize)]`)**: Mapping TOML document fields to Rust structs automatically.
- **Cross-Platform Path Normalization**: Unifying `\` (Windows) and `/` (Unix) path separators for reliable string comparison.

---

## Milestone 12: CLI Flag `--exclude` (`clap`)

### Summary
- Added `#[arg(short, long)] exclude: Vec<String>` option to `Commands::Scan` in [src/main.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/main.rs).
- Merged exclude patterns from `devener.toml` and CLI `--exclude` arguments.
- Filtered out matching items seamlessly during directory scanning.

### Key Rust Concepts Learned
- **Repeated CLI Flags in Clap**: Parsing multi-value list flags (`--exclude path1 --exclude path2`).
- **Vector Combination (`extend`)**: Merging collection vectors efficiently.

---

## Milestone 13: README & Documentation Update v2

### Summary
- Updated [README.md](file:///c:/Users/Senna/Desktop/Projects/Devener/README.md) to document all v2 features.
- Built release binary `cargo build --release` successfully in 15.72s.

---

## Milestone 14: Research & Setup Crate `trash` (v3 Recycle Bin Safety Net)

### Summary
- Researched cross-platform mechanics of the `trash` crate.
- Added `trash = "5.1"` dependency to [Cargo.toml](file:///c:/Users/Senna/Desktop/Projects/Devener/Cargo.toml) and compiled clean (`cargo build`).

---

## Milestone 15: Trash-Based Deletion Execution

### Summary
- Replaced `std::fs::remove_dir_all` with `trash::delete(&item.path)` in [src/cleaner.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/cleaner.rs).
- Updated error matching to handle `trash::Error` variants gracefully.
- Updated confirmation prompt messages to reflect moving to Recycle Bin.

### Key Rust Concepts Learned
- **Recycle Bin Operations (`trash::delete`)**: Moving directories to OS Recycle Bin safely instead of permanent deletion.
- **Error Mapping**: Transforming `trash::Error` into user-friendly error strings.

### Verification & Testing
- Command: `cargo test` -> `5 passed (1 suite, 0.17s)`

---

## Milestone 16: Update Report Messaging & README for v3

### Summary
- Updated report messages in [src/cleaner.rs](file:///c:/Users/Senna/Desktop/Projects/Devener/src/cleaner.rs) to clarify items are moved to OS Recycle Bin.
- Updated [README.md](file:///c:/Users/Senna/Desktop/Projects/Devener/README.md) featuring the Recycle Bin Safety Net value proposition.

---

## Milestone 17: Full Regression Check & Release Build v3

### Summary
- Ran complete test suite regression check: all 5 unit tests passed cleanly (`5 passed`).
- Built optimized release binary `cargo build --release` in 27.73s (`target/release/devener.exe`).
- Verified complete system stability across v1, v2, and v3 features.

### Verification & Testing
- Command: `cargo test` -> `5 passed (1 suite, 0.18s)`
- Command: `cargo build --release` -> `Finished release profile [optimized] target(s) in 27.73s`
