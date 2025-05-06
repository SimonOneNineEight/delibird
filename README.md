# Delibird

![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)

A terminal-based todo list application built with [Ratatui](https://ratatui.rs), providing a clean and efficient interface for managing your tasks.

## Features

- **Simple Task Management**: Create, toggle, and navigate through your tasks with ease
- **Keyboard-Driven Interface**: Fast and intuitive keyboard shortcuts for all operations
- **Beautiful TUI**: Clean, modern terminal UI with customized styling
- **Persistent Storage**: (Coming soon) Save your tasks between sessions

<!-- ## Screenshots -->
<!---->
<!-- *Screenshots will be added here* -->

## Installation

### Prerequisites

- Rust and Cargo (2024 Edition)

### From Source

```bash
# Clone the repository
git clone https://github.com/SimonOneNineEight/delibird.git
cd delibird

# Build and run
cargo run
```

### Cargo Install

```bash
# Install directly from crates.io (Coming soon)
cargo install delibird
```

## Usage

Once launched, you can use the following keyboard shortcuts:

| Key      | Action               |
| -------- | -------------------- |
| `n`      | Create a new task    |
| `j`      | Navigate down        |
| `k`      | Navigate up          |
| `Enter`  | Toggle selected task |
| `Esc`    | Exit edit mode       |
| `q`      | Quit application     |
| `Ctrl+C` | Quit application     |

### Adding Tasks

1. Press `n` to enter edit mode
2. Type your task description
3. Press `Enter` to save the task

### Completing Tasks

1. Navigate to the task using `j` and `k`
2. Press `Enter` to toggle completion status

## Project Structure

The application follows a modular architecture:

- `app.rs`: Main application state and event handling
- `event.rs`: Event handling logic
- `input.rs`: Input field management
- `task.rs`: Task data structures and operations
- `ui.rs`: User interface rendering
- `main.rs`: Application entry point

<!-- ## Contributing -->
<!---->
<!-- Contributions are welcome! Feel free to: -->
<!---->
<!-- 1. Fork the repository -->
<!-- 2. Create a feature branch (`git checkout -b feature/amazing-feature`) -->
<!-- 3. Commit your changes (`git commit -m 'Add some amazing feature'`) -->
<!-- 4. Push to the branch (`git push origin feature/amazing-feature`) -->
<!-- 5. Open a Pull Request -->
<!---->
<!-- Please make sure to update tests as appropriate and adhere to the existing coding style. -->

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Acknowledgments

- [Ratatui](https://ratatui.rs) - The TUI framework this application is built upon
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [Event-driven template](https://github.com/ratatui/templates/tree/main/event-driven) - The template used to bootstrap this project

---

Created by [SimonOneNineEight](https://github.com/SimonOneNineEight)
