# Claude Projects Viewer

An interactive TUI application to search and browse Claude projects stored in `~/.claude/projects/**/*.jsonl` files.

## Features

- Load and parse all Claude project JSONL files (latest 30 for performance)
- Real-time search across all messages
- Interactive navigation with arrow keys
- View detailed message content
- Fullscreen TUI with beautiful table display
- Color-coded message types (user, assistant, system)
- Static output mode for copying results

## How to Run

### Interactive Mode (default)
```bash
cargo run
```

### Static Output Mode (copyable)
```bash
# Show all projects with detailed output
cargo run --bin claude-projects-viewer-print

# Search with a query
cargo run --bin claude-projects-viewer-print "rust"
```

### Alternative Version (manual text input)
```bash
# If TextInput doesn't show characters properly
cargo run --bin claude-projects-viewer-alt
```

## Controls

### Interactive Mode
- **Tab**: Toggle focus between search input and results list
- **↑/↓**: Navigate through search results (when list has focus)
- **Enter**: View detailed messages of selected project
- **ESC**: Go back to list view or exit application
- **Type to search**: Enter text when search input has focus

### Alternative Version (Alt)
- Same as above, plus:
- **Backspace**: Delete last character in search (when search has focus)

## Known Issues

- TextInput may not display entered characters on some terminal configurations
- Use the alternative version (`--bin claude-projects-viewer-alt`) if you experience this issue
- The static output mode (`--bin claude-projects-viewer-print`) provides copyable text output

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