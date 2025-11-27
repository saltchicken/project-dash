# Project Dash

![Rust](https://img.shields.io/badge/Made_with-Rust-orange?style=flat-square&logo=rust)
![Crates.io](https://img.shields.io/crates/v/project-dash?style=flat-square&color=blue)
![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)

**Project Dash** is a lightweight, keyboard-centric terminal user interface (TUI) tool designed to streamline directory navigation. It scans your `~/Desktop` directory, allowing you to filter, select, and output folder paths directly to standard output. Built for speed and efficiency, it integrates seamlessly into shell workflows.

## 🚀 Features

-   **Desktop Scanning**: Automatically detects and lists non-hidden directories on your Desktop.
-   **Interactive TUI**: A responsive interface built with `ratatui` and `crossterm`.
-   **Real-time Filtering**: Type to filter directories instantly (fuzzy-search style logic).
-   **Vim-like Navigation**: Supports `j`/`k` navigation alongside standard arrow keys.
-   **Dual Operation Modes**:
    -   **Normal Mode**: For navigation and selection.
    -   **Editing Mode**: For typing filter queries.
-   **Shell Integration**: Outputs the absolute path of the selected directory to `stdout` on exit, making it pipeable to other commands (e.g., `cd $(project-dash)`).

## 🛠 Tech Stack

-   **Language**: Rust (Edition 2024)
-   **UI Framework**: [Ratatui](https://github.com/ratatui-org/ratatui) (v0.29.0)
-   **Terminal Backend**: Crossterm
-   **Async Runtime**: Tokio
-   **Error Handling**: Color-Eyre

## 📋 Prerequisites

Ensure you have the following installed on your system:

-   **Rust & Cargo**: Version 1.75 or higher (due to 2024 edition usage).
-   **Terminal**: A terminal emulator supporting ANSI escape codes.

## 📦 Installation

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/yourusername/project-dash.git](https://github.com/yourusername/project-dash.git)
    cd project-dash
    ```

2.  **Build the project:**
    ```bash
    cargo build --release
    ```

3.  **(Optional) Install globally:**
    ```bash
    cargo install --path .
    ```

## 💻 Usage

Run the application directly via Cargo or the binary:

```bash
cargo run --release
```

### Shell Integration Example
To use Project Dash to change your current directory, add this alias to your `.bashrc` or `.zshrc`:

```bash
alias d="cd \$(/path/to/project-dash)"
```

### Keyboard Controls

The application operates in two modes: **Normal** and **Editing**.

| Key | Action | Mode Context |
| :--- | :--- | :--- |
| **Normal Mode** | (Blue Border) | |
| `j` / `↓` | Select next item | Navigation |
| `k` / `↑` | Select previous item | Navigation |
| `/` | Enter **Editing Mode** | Search |
| `Enter` | Confirm selection | Submit |
| `q` | Quit application | Exit |
| **Editing Mode** | (Yellow Border) | |
| `Esc` | Return to **Normal Mode** | Cancel Input |
| `Enter` | Confirm selection | Submit |
| `Typing...` | Filter the list | Search |
| `Backspace` | Delete character | Search |
| `↑` / `↓` | Navigate list | Navigation |

## ⚙️ Configuration

The application currently defaults to scanning the `~/Desktop` directory. This behavior is defined in `src/app/fs.rs`.

Environment variables used:
-   `HOME`: Used to locate the user's home directory and subsequent Desktop path.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1.  Fork the project
2.  Create your feature branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request
            
