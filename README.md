# EntropUI

**EntropUI** is a lightweight desktop GUI built in Rust using [`eframe`](https://github.com/emilk/egui/tree/master/crates/eframe) / [`egui`](https://github.com/emilk/egui) to automate and run Permutation Entropy and Statistical Complexity ($H \times C$) computations backed by existing MATLAB scripts.

---

## Features

- **Native Desktop Interface**: Fast, immediate-mode GUI written in Rust (`eframe`/`egui`) with multi-page navigation.
- **Entropy & Complexity ($H \times C$) Computation**:
  - Configurable **Data Path**, **Embedding Dimension** ($m$), and **Embedding Time Delay** ($t$).
  - Calls the MATLAB `HXC_standard_deviation` routine automatically.
- **Non-Blocking Execution**: Runs MATLAB in a dedicated background thread and streams `stdout` live into the application console using `std::sync::mpsc` channels.
- **Interactive Plot Support**: Automatically keeps generated MATLAB figures open (`waitfor(gcf)`) until manually closed by the user.

---

## Prerequisites

- **OS**: Linux *(currently targeted; Windows/macOS support planned)*
- **Rust**: Stable toolchain (`rustc` and `cargo`) — install via [rustup.rs](https://rustup.rs/)
- **MATLAB**: Installed and accessible from the command line (`matlab` must be in your system `PATH`)
- **MATLAB Scripts**: Local copy of the `HXC_standard_deviation` MATLAB functions

---

## Configuration

Before running the application, update the path to your local MATLAB `HXC` scripts directory in `src/compute_hxc.rs`:

```rust
let matlab_scripts_dir = "/path/to/your/HXC/scripts";
```

Verify that `matlab` can be launched from your terminal:

```bash
which matlab
```

---

## Getting Started

1. **Clone the repository**:
   ```bash
   git clone <your-repo-url>
   cd <your-repo-directory>
   ```

2. **Build and run the application**:
   ```bash
   cargo run
   ```

   For an optimized build:
   ```bash
   cargo run --release
   ```

---

## Usage

1. Launch **EntropUI** and navigate to **Compute Entropy and Complexity** from the startup screen or top navigation bar.
2. Enter the path to your dataset in **Data Path** (e.g., `/path/to/dataset`).
3. Adjust the **Embedding Dimension** ($m$) and **Embedding Time Delay** ($t$).
4. Click **Run MATLAB**.
   - The button disables automatically while MATLAB is running.
   - Console logs stream in real time in the **MATLAB output** panel.
   - If the script generates a figure, the process waits until the figure window is closed before terminating MATLAB.

---

## Project Structure

```text
src/
├── main.rs          # Entry point & native OS window initialization ("EntropUI")
├── app.rs           # Main application state (MyApp), navbar, and page routing
├── startup_page.rs  # Landing page
├── compute_hxc.rs   # HxC computation UI & background MATLAB process runner
├── example.rs       # Example / test UI component (Test 1)
└── page2.rs         # Secondary / test UI component (Test 2)
```

---

## Roadmap

- [ ] Replace the hardcoded MATLAB scripts path with a UI file/folder picker or configuration file.
- [ ] Add a native file dialog for selecting the dataset path.
- [ ] Capture and display `stderr` alongside `stdout` in the output console.
- [ ] Clean up placeholder pages (`Test 1`, `Test 2`) as new analysis tools are added.
- [ ] Extend cross-platform support (Windows / macOS).