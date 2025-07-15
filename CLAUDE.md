# CLAUDE.md

Do not write any code. Either provide a detailed plan for me to write the code, or provide the code itself and explain it. I'm learning Rust so I want to understand the concepts, and instead of just writing the code for me, make it so that it's // TO DO: for me to fill in the code myself, do not write any of the code yourself unless I ask you to, just provide the skeleton. Never add emojis as well.

## Code Assistance Style

**IMPORTANT**: When providing code help, use detailed TODO comments that emphasize OPTIMIZED and EFFICIENT implementations:

### Optimization-Focused Example:
```rust
fn validate_answer(&self, expected: &ParsedSchema) -> ValidationResult {
    // TO DO: Pre-allocate HashMap with estimated capacity using HashMap::with_capacity()
    // TO DO: Create empty vectors for missing_nodes and extra_nodes with Vec::new()
    
    // TO DO: Convert self.nodes.keys() to HashSet<String> using .cloned().collect()
    // OPTIMIZATION: Consider using references instead of cloning if possible
    // TO DO: Convert expected.nodes.keys() to HashSet<String> using .cloned().collect()
    
    // TO DO: Use .difference() method - this is O(n) and efficient for set operations
    // TO DO: Collect missing nodes using .cloned().collect() into Vec
    
    // TO DO: Use .difference() in reverse to find extra nodes efficiently
    // TO DO: Collect extra nodes using .cloned().collect() into Vec
    
    // TO DO: Use .intersection() method for O(min(m,n)) performance
    // TO DO: Iterate through common nodes and compare HashSets directly (efficient comparison)
    // OPTIMIZATION: HashSet equality is O(n) and well-optimized in Rust
    
    // TO DO: Only create PropertyErrors when properties don't match (avoid unnecessary allocations)
    // TO DO: Use HashMap::insert() only when needed
    
    // TO DO: Calculate is_correct using .is_empty() methods (O(1) operations)
    // TO DO: Return ValidationResult - struct initialization is zero-cost
}
```

### Performance-Focused Parsing Example:
```rust
fn parse(content: &str) -> Result<Self, String> {
    // TO DO: Initialize HashMap with HashMap::new() (grows as needed)
    // TO DO: Create iterator with content.lines().map(str::trim) for lazy evaluation
    // OPTIMIZATION: Iterator avoids collecting all lines into memory at once
    
    // TO DO: Use while let Some(line) = lines.next() for efficient iteration
    // TO DO: Use strip_prefix("N::") instead of manual string slicing (safer + faster)
    
    // TO DO: Use find('{') to locate opening brace (efficient string search)
    // TO DO: Use string slicing [..bracket_pos] for zero-copy substring
    
    // TO DO: Initialize HashSet with HashSet::new() for properties
    // TO DO: Use for loop with &mut lines to consume iterator efficiently
    
    // TO DO: Use split_once(':') instead of split().collect() (more efficient)
    // OPTIMIZATION: split_once only finds first occurrence, avoiding vector allocation
    // TO DO: Use trim() on both parts before creating Property struct
    
    // TO DO: Use HashSet::insert() which handles deduplication automatically
    // TO DO: Use HashMap::insert() to store node with its properties
}
```

### Bad Example (Inefficient):
```rust
fn validate_answer(&self, expected: &ParsedSchema) -> ValidationResult {
    let all_nodes = Vec::new();
    for node in self.nodes.keys() {
        all_nodes.push(node.clone()); // Unnecessary vector allocation
    }
    // ... (using inefficient nested loops instead of set operations)
}
```

**Key Optimization Principles**:
- **Memory Efficiency**: Use iterators over collections, avoid unnecessary allocations
- **Time Complexity**: Choose O(1) and O(n) operations over O(n²) when possible
- **Zero-Cost Abstractions**: Leverage Rust's iterator chains and method chaining
- **Borrowing Over Ownership**: Use references when cloning isn't necessary
- **Lazy Evaluation**: Use iterators that process data on-demand
- **Appropriate Data Structures**: HashSet for uniqueness, HashMap for key-value lookups

## Optimization Guidelines for Learning

### Memory Optimization
- **TO DO Comments should emphasize**: 
  - Using `Vec::with_capacity()` when size is known
  - Choosing `&str` over `String` when possible
  - Using iterator chains instead of intermediate collections
  - Leveraging `split_once()` over `split().collect()`

### Performance Patterns
- **TO DO Comments should teach**:
  - Set operations (.difference(), .intersection()) for O(n) comparisons
  - `strip_prefix()` and `starts_with()` for efficient string processing
  - Pattern matching with `if let` for cleaner control flow
  - Using `map_err()` for error transformation without panic

### Rust-Specific Optimizations
- **Emphasize in TODOs**:
  - Iterator adapters are zero-cost abstractions
  - `HashMap` and `HashSet` equality is highly optimized
  - String slicing is zero-copy when borrowing
  - `match` expressions compile to efficient jump tables

### Anti-Patterns to Avoid
- **TODOs should warn against**:
  - Manual indexing when iterators are available
  - Collecting iterators unnecessarily 
  - Using `clone()` when `&` references suffice
  - Nested loops when set/map operations exist
  - `expect()` and `unwrap()` in production code

## Project Overview

This is a Rust-based CLI application called "helixir" - a rustling-styled interactive learning tool that teaches users how to use helix-db from 0 to hero. The CLI guides users through lessons covering core helix-db concepts including schema design and query writing, with emphasis on writing PERFORMANT and OPTIMIZED code.

## Architecture

- **helixir-cli/**: Main CLI application written in Rust with performance focus
  - `src/main.rs`: Entry point optimized for fast startup
  - `src/validation.rs`: Efficient parsing and validation engine
  - `Cargo.toml`: Project configuration using Rust 2024 edition with optimization flags

## Learning Objectives

This CLI teaches users to write OPTIMIZED helix-db code:
- **Efficient** HelixDB schema design using nodes (N::) and edges (E::)
- Writing **performant** creation queries (AddN, AddE operations)
- Writing **optimized** read queries (traversal patterns)
- Understanding helix-db data modeling with **memory efficiency**
- Working with geographic data model using **zero-copy** string operations

## Development Commands

### Building with Optimizations
```bash
cd helixir-cli
cargo build --release  # Always test with optimizations
```

### Performance Profiling
```bash
cd helixir-cli
cargo run --release    # Run optimized builds
```

### Testing with Benchmarks
```bash
cd helixir-cli
cargo test --release   # Test optimized code paths
```

### Linting for Performance
```bash
cd helixir-cli
cargo clippy -- -W clippy::perf          # Performance lints
cargo clippy -- -W clippy::nursery       # Advanced optimizations
cargo fmt
```

## Project Structure

The repository contains a single Rust CLI project in the `helixir-cli/` directory. This rustling-styled CLI will provide interactive lessons teaching helix-db concepts through hands-on exercises that emphasize PERFORMANCE and EFFICIENCY.

## Lesson Structure

The CLI will guide users through progressive lessons with optimization focus:
1. **Efficient Schema Design**: Understanding nodes and edges with optimal data structures
2. **Performant Creation Queries**: Building data with minimal allocations
3. **Optimized Read Queries**: Fast traversals and efficient data retrieval
4. **Advanced Performance Patterns**: Zero-copy operations and memory optimization

Each lesson builds on the previous one, starting from basic concepts and progressing to advanced helix-db usage patterns with performance optimization as a core principle.

## Technical Architecture

### Core Types (Optimized Design)
- `MenuAction`: Command enumeration with exhaustive matching (zero-cost)
- `Lesson`: Complete lesson data structure with minimal heap allocations
- `ValidationResult`: Comprehensive validation feedback with efficient error handling
- `FileType`: Strongly typed file categorization using enums (stack allocation)

### Module Structure (Performance-Focused)
- `main.rs`: Application entry point with fast startup optimizations
- `lesson.rs`: Lesson management with lazy loading patterns
- `validation.rs`: High-performance file parsing and comparison engine  
- `cli.rs`: Command interface with minimal terminal I/O overhead

### Error Handling Strategy (Efficient)
- Use `Result<T, E>` for all fallible operations (zero-cost when successful)
- Implement custom error types with `thiserror` for efficient error propagation
- Provide actionable error messages using `format!` only when needed
- Graceful degradation using iterators and combinators for performance