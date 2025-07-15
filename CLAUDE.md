# CLAUDE.md

Do not write any code. Either provide a detailed plan for me to write the code, or provide the code itself and explain it. I'm learning Rust so I want to understand the concepts, and instead of just writing the code for me, make it so that it's // TO DO: for me to fill in the code myself, do not write any of the code yourself unless I ask you to, just provide the skeleton. Never add emojis as well.

## Code Assistance Style

**IMPORTANT**: When providing code help, use detailed TODO comments instead of writing implementation:

### Good Example:
```rust
fn validate_answer(&self, expected: &ParsedSchema) -> ValidationResult {
    // TO DO: Create empty vectors for missing_nodes and extra_nodes
    // TO DO: Create empty HashMap for property_errors
    
    // TO DO: Convert self.nodes.keys() to HashSet<String> using .cloned().collect()
    // TO DO: Convert expected.nodes.keys() to HashSet<String> using .cloned().collect()
    
    // TO DO: Use .difference() method to find nodes in expected but not in user
    // TO DO: Push each missing node name to missing_nodes vector
    
    // TO DO: Use .difference() method to find nodes in user but not in expected  
    // TO DO: Push each extra node name to extra_nodes vector
    
    // TO DO: Use .intersection() method to find common nodes
    // TO DO: For each common node, compare property HashSets
    // TO DO: If properties don't match, create PropertyErrors and add to property_errors HashMap
    
    // TO DO: Calculate is_correct: true if all error vectors/maps are empty
    // TO DO: Return ValidationResult with all the collected data
}
```

### Bad Example:
```rust
fn validate_answer(&self, expected: &ParsedSchema) -> ValidationResult {
    let mut missing_nodes = Vec::new();
    let user_nodes: HashSet<String> = self.nodes.keys().cloned().collect();
    // ... (providing too much implementation)
}
```

**Key Principles**:
- Provide detailed step-by-step TODO comments
- Explain the Rust concepts and methods to use
- Give context about what data structures and operations are needed
- Let me implement the actual code to learn the language
- Focus on teaching Rust patterns and best practices through the TODOs

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


## Technical Architecture

### Core Types
- `MenuAction`: Command enumeration with exhaustive matching
- `Lesson`: Complete lesson data structure with metadata
- `ValidationResult`: Comprehensive validation feedback system
- `FileType`: Strongly typed file categorization

### Module Structure
- `main.rs`: Application entry point and main loop
- `lesson.rs`: Lesson management and progression logic
- `validation.rs`: File parsing and comparison engine  
- `cli.rs`: Command interface and terminal operations

### Error Handling Strategy
- Use `Result<T, E>` for all fallible operations
- Implement custom error types for domain-specific failures
- Provide actionable error messages with context
- Graceful degradation for non-critical failures