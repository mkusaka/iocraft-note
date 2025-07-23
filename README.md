# iocraft Search UI Demo

A product search UI demo application built with the iocraft library.

## Features

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

## Technologies Used

- [iocraft](https://github.com/ccbrown/iocraft) - A React-like declarative TUI library
- Rust
- serde/serde_json - JSON data loading

## Screenshots

The application displays products in a clean table format with headers showing Product Name, Brand, Category, Price, and Stock status. Selected rows are highlighted with a green border, and alternating row colors improve readability.