# CLAUDE.md

## Project Overview

This is a Rust-based CLI application called "helixir" - a rustling-styled interactive learning tool that teaches users how to use helix-db from 0 to hero. The CLI guides users through lessons covering core helix-db concepts including schema design and query writing.

## Architecture

- **helixir-cli/**: Main CLI application written in Rust
  - `src/main.rs`: Entry point with basic "Hello, world!" implementation
  - `Cargo.toml`: Project configuration using Rust 2024 edition

## Learning Objectives

This CLI teaches users:
- HelixDB schema design using nodes (N::) and edges (E::)
- Writing creation queries (AddN, AddE operations)
- Writing read queries (traversal patterns)
- Understanding helix-db data modeling patterns
- Working with the geographic data model (continents → countries → cities)

## Development Commands

### Building
```bash
cd helixir-cli
cargo build
```

### Running
```bash
cd helixir-cli
cargo run
```

### Testing
```bash
cd helixir-cli
cargo test
```

### Linting/Formatting
```bash
cd helixir-cli
cargo clippy
cargo fmt
```

## Project Structure

The repository contains a single Rust CLI project in the `helixir-cli/` directory. This rustling-styled CLI will provide interactive lessons teaching helix-db concepts through hands-on exercises based on the quickstart tutorial.

## Lesson Structure

The CLI will guide users through progressive lessons:
1. **Schema Design**: Understanding nodes and edges in the geographic model
2. **Creation Queries**: Building data with AddN and AddE operations
3. **Read Queries**: Traversing relationships and retrieving data
4. **Advanced Patterns**: Complex queries and data relationships

Each lesson builds on the previous one, starting from basic concepts and progressing to advanced helix-db usage patterns.