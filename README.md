# Word Puzzles API

A Rust-based API service that provides word puzzle solving capabilities and word search functionality. This project serves as both a practical tool for word games and a technical interview resource.

## Features

- Word search and validation via REST API
- Interactive word puzzles using HTMX
- In-memory word storage

## API Endpoints

### Word Search

- `GET /api/search?q={word}` - Search for words matching the query

  - Returns a list of matching words
  - Case-insensitive search

## Web Interface

The project includes an interactive web interface built with HTMX for:

- Word puzzles

## Technical Requirements

- Rust 1.70 or higher

## Getting Started

1. Clone the repository:

```bash
git clone https://github.com/yourusername/word_puzzles.git
cd word_puzzles
```

2. Run the server:

```bash
cargo run web
```

The server will start on `http://localhost:8080` by default.

## API Usage Examples

### Search for words

```bash
curl "http://localhost:8080/api/search?q=hello"
```
