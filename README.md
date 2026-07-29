# devener 🧹 (v3)

**Dev Environment Cleaner CLI** — Tool CLI serbaguna berbasis Rust untuk menemukan dan memindahkan folder build artifacts & cache raksasa yang tidak terpakai di komputer Anda ke Recycle Bin secara aman.

---

## ✨ Fitur & Value Proposition (v3)

- **🛡️ Recycle Bin / Trash Safety Net (NEW in v3)**: Tidak menghapus file secara permanen! Folder yang dibersihkan dipindahkan ke **Recycle Bin (Windows)** / **Trash (macOS)** / **XDG Trash (Linux)**. Jika Anda salah memilih folder, Anda dapat dengan mudah melakukan **Restore** dari Recycle Bin OS Anda kapan saja.
- **Multi-Ecosystem Expansion**: Mendeteksi berbagai jenis artifact sekaligus dalam satu kali scan:
  - **JavaScript / TypeScript**: `node_modules`, `dist`, `build`, `.next`
  - **Python**: `__pycache__`, `.venv`
  - **Rust / Cargo**: `target`
  - **C / C++ / CMake**: `cmake-build-debug`, `cmake-build-release`, `CMakeFiles`
  - **iOS / macOS / Xcode & CocoaPods**: `DerivedData`, `Pods`
  - **Java / Kotlin / Gradle**: `.gradle`
  - **Go**: `vendor`
- **Context-Aware Detection**: Mencegah *false positive* pada folder dengan nama umum:
  - `.NET / C#` (`bin`, `obj`): Hanya di-match jika ada file `.csproj` atau `.sln` di folder induknya.
  - `Unity` (`Library`): Hanya di-match jika folder `Assets` DAN `ProjectSettings` ada di sebelahnya.
- **Progress Indicator Real-Time**: Spinner dan penghitung progress terintegrasi menggunakan `indicatif`.
- **Custom Exclude via `devener.toml` & CLI Flag `--exclude`**: Abaikan direktori atau project tertentu secara mudah.
- **Safety-First**: Menampilkan prompt interaktif checkbox & konfirmasi final sebelum tindakan diambil.

---

## ⚙️ Pengaturan File Konfigurasi `devener.toml`

Anda dapat membuat file `devener.toml` di direktori kerja untuk mengabaikan folder/project tertentu dari hasil scan:

```toml
# contoh devener.toml
exclude = [
    "./important-project/node_modules",
    "./keep-this/target"
]
```

---

## 📽️ Demo & Preview Output

<!-- 
  PLACEHOLDER DEMO GIF / SCREENSHOT
  Instruksi untuk Maintainer: 
  Rekam terminal menggunakan VHS/asciinema atau ambil screenshot tampilan terminal devener v3, 
  lalu simpan file ke docs/demo.gif dan perbarui link markdown di bawah ini.
-->
![devener v3 Demo](docs/demo.gif)

### Contoh Tampilan CLI v3

```text
> devener scan

Scanning directory: .

Ditemukan 4 folder yang bisa dibersihkan:

  [1] DerivedData     ./ios_app/DerivedData           2.40 GB
  [2] node_modules    ./web_app/node_modules          1.24 GB
  [3] target          ./rust_app/target                340.50 MB
  [4] .venv           ./python_app/.venv               210.12 MB

Total potensi space yang bisa direclaim: 4.19 GB

? Pilih folder yang ingin dibersihkan (Gunakan SPACE untuk memilih, ENTER untuk konfirmasi): 
  [x] DerivedData     ./ios_app/DerivedData           2.40 GB
  [x] node_modules    ./web_app/node_modules          1.24 GB
  [x] target          ./rust_app/target                340.50 MB
  [ ] .venv           ./python_app/.venv               210.12 MB

? Anda akan memindahkan 3 folder ke Recycle Bin, total 3.98 GB — lanjutkan? Yes

Memindahkan folder terpilih ke Recycle Bin...
  ✔ ./ios_app/DerivedData
  ✔ ./web_app/node_modules
  ✔ ./rust_app/target

=== RINGKASAN PEMBERSIHAN ===
✔ Berhasil memindahkan 3 folder ke Recycle Bin
✔ Space yang berhasil direclaim: 3.98 GB
(Folder tersimpan aman di Recycle Bin OS dan dapat di-restore jika diperlukan)
```

---

## 🚀 Cara Instalasi

### 1. Build & Install dari Source (menggunakan Cargo)

Pastikan Anda memiliki Rust & Cargo terinstall di sistem Anda, lalu jalankan:

```bash
git clone https://github.com/username/devener.git
cd devener
cargo install --path .
```

Setelah terinstall, perintah `devener` dapat dipanggil langsung dari terminal mana saja.

### 2. Menjalankan Langsung via Cargo

```bash
cargo run -- scan [PATH] [--exclude <path>]
```

---

## 📖 Cara Penggunaan

### Scan & Exclude via Command Line

```bash
# Scan direktori saat ini
devener scan

# Scan dengan flag exclude
devener scan --exclude ./my-app/node_modules --exclude ./rust/target
```

---

## 🛠️ Pengembangan (Development)

```bash
# Kompilasi project
cargo build

# Menjalankan unit test suite lengkap
cargo test
```

---

## 📜 Lisensi

MIT License © 2026
