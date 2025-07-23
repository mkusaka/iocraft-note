# iocraft Demo Applications

This repository contains demo applications built with the iocraft library.

## Projects

### 1. Product Search UI

A product search UI demo application located in the root directory.

#### Features

- Product search with filtering by name, category, and brand
- Navigation with arrow keys
- Display product details with Enter key
- Return to previous screen or exit application with ESC key
- Beautiful table-style product list display
- Fullscreen mode for optimal screen utilization

## How to Run

```bash
cargo run
```

## Controls

- **Search**: Type in the text box to filter by product name, category, or brand
- **↑/↓**: Move selection within the list
- **Enter**: Display details of the selected product
- **ESC**: Return to list view from detail view, or exit application from list view

### 2. Claude Projects Viewer

An interactive TUI to search and browse Claude projects stored in JSONL files.

Located in the `claude-projects-viewer` directory.

#### Features

- Load and parse all Claude project JSONL files from `~/.claude/projects/**/*.jsonl`
- Real-time search across all messages
- View detailed message content with color-coded message types
- Interactive navigation with fullscreen TUI

#### How to Run

```bash
cd claude-projects-viewer
cargo run
```

#### Controls

- **Tab**: Toggle focus between search input and results list
- **↑/↓**: Navigate through search results (when list has focus)
- **Enter**: View detailed messages of selected project
- **ESC**: Go back to list view or exit application
- **Type to search**: Enter text when search input has focus

## Technologies Used

- [iocraft](https://github.com/ccbrown/iocraft) - A React-like declarative TUI library
- Rust
- serde/serde_json - JSON/JSONL data parsing
- glob - File pattern matching
- home - Home directory detection