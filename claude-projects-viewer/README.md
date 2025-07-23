# Claude Projects Viewer

An interactive TUI application to search and browse Claude projects stored in `~/.claude/projects/**/*.jsonl` files.

## Features

- Load and parse all Claude project JSONL files
- Real-time search across all messages
- Interactive navigation with arrow keys
- View detailed message content
- Fullscreen TUI with beautiful table display
- Color-coded message types (user, assistant, system)

## How to Run

```bash
cargo run
```

## Controls

- **Search**: Type in the search box to filter messages across all projects
- **↑/↓**: Navigate through search results
- **Enter**: View detailed messages for selected project
- **ESC**: Go back to search results or exit application

## Technologies Used

- [iocraft](https://github.com/ccbrown/iocraft) - React-like declarative TUI library
- Rust
- serde/serde_json - JSON/JSONL parsing
- glob - File pattern matching
- home - Home directory detection

## Architecture

- `types.rs`: Type definitions based on ccmeta schemas
- `parser.rs`: JSONL file parsing and search functionality  
- `main.rs`: Interactive TUI implementation

The application parses Claude's session message format, supporting various message types including user messages, assistant responses, system messages, and tool use results.