# Delibird

![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)

A terminal-based todo list application built with [Ratatui](https://ratatui.rs), providing a clean and efficient interface for managing your tasks.

## Features

- **Simple Task Management**: Create, toggle, and navigate through your tasks with ease
- **Keyboard-Driven Interface**: Fast and intuitive keyboard shortcuts for all operations
- **Beautiful TUI**: Clean, modern terminal UI with customized styling
- **Persistent Storage**: Your tasks are automatically saved between sessions
- **Task Prioritization**: Mark tasks as favorites to keep them at the top of your list
- **Task Status**: Easily track completed and pending tasks with visual indicators

## Key Components

- **Task Management**: Create, complete, delete and prioritize tasks
- **Favorite Tasks**: Star important tasks to keep them at the top of your list
- **Visual Indicators**: Different colors for completed tasks and favorites
- **Automatic Saving**: Tasks are automatically saved to your config directory

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

| Key      | Action                     |
| -------- | -------------------------- |
| `n`      | Create a new task          |
| `j`      | Navigate down              |
| `k`      | Navigate up                |
| `Enter`  | Toggle task completion     |
| `d`      | Delete selected task       |
| `s`      | Toggle favorite status     |
| `h`      | Show help popup            |
| `Esc`    | Exit edit mode/Close popup |
| `q`      | Quit application           |
| `Ctrl+C` | Quit application           |

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

## Data Storage

Delibird stores your tasks in JSON format at:

- Linux/macOS: `~/.config/delibird/tasks.json`
- Windows: `%APPDATA%\delibird\tasks.json`

Tasks are automatically saved whenever you make changes.

## Planned Features

- Task categories/tags
- Due dates
- Priority levels
- Search functionality

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Acknowledgments

- [Ratatui](https://ratatui.rs) - The TUI framework this application is built upon
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [Event-driven template](https://github.com/ratatui/templates/tree/main/event-driven) - The template used to bootstrap this project

---

Created by [SimonOneNineEight](https://github.com/SimonOneNineEight)
