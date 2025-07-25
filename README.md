# Dice Rust

A Rust library and command-line tool for parsing dice notation expressions with interpreter support.

## Overview

Dice Rust is a comprehensive parser and execution environment for dice notation, commonly used in tabletop role-playing games. It can parse expressions like "2d6" (roll 2 six-sided dice) or "3d20" (roll 3 twenty-sided dice) and execute them on a stack-based virtual machine.

## Features

- **Lexical Analysis**: Tokenizes dice notation strings into meaningful tokens
- **Parsing**: Converts token streams into a structured AST
- **Semantic Analysis**: Validates the AST for correctness
- **Stack VM**: Interpret dice expressions on a virtual machine
- **Error Handling**: Comprehensive error reporting with position information
- **Type Safety**: Built with Rust's type system for reliability
- **Pure Rust**: No external dependencies for core functionality

## Dice Notation Format

The parser supports the standard dice notation format:

- `NdS` where `N` is the number of dice and `S` is the number of sides
- Examples: `1d6`, `2d10`, `3d20`, `1d100`
- Both lowercase `d` and uppercase `D` are supported

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building from Source

```bash
git clone https://github.com/japan-expert-force/dice-rust.git
cd dice-rust
cargo build --release
```

## Usage

Execute dice expressions using the built-in virtual machine:

```bash
# Run with default expression (2d100)
cargo run -- run

# Run with custom expression
cargo run -- run "3d6"
cargo run -- run "1d20"
```

### Examples

```bash
# Stack VM execution
$ cargo run -- run "2d6"
5
2
Total: 7
```

## Project Structure

```
src/
├── analyzer.rs     # Semantic analysis
├── ast.rs         # Abstract Syntax Tree definitions
├── error.rs       # Error types and handling
├── lexer.rs       # Lexical analysis
├── lib.rs         # Library interface
├── main.rs        # CLI interface
├── parser.rs      # Syntax analysis
└── stack_vm.rs    # Stack-based virtual machine
```

````

## Architecture

### Components

1. **Lexer** (`lexer.rs`): Converts input strings into tokens

   - Recognizes numbers, dice operators (`d`/`D`), and end-of-file
   - Handles position tracking for error reporting

2. **Parser** (`parser.rs`): Converts token streams into AST

   - Implements recursive descent parsing
   - Validates syntax and structure

3. **AST** (`ast.rs`): Defines the structure of parsed expressions

   - `Program`: Root node containing statements
   - `Statement`: Expression statements
   - `Expression`: Dice expressions with count and faces

4. **Error Handling** (`error.rs`): Comprehensive error types
   - Position-aware error reporting
   - Detailed error messages for debugging

### Example AST Output

For the input "2d100", the parser generates:

```rust
Program {
    statement: Some(Statement {
        kind: Expression {
            expr: Expression {
                kind: Dice { count: 2, faces: 100 },
                span: Span { /* position info */ }
            }
        },
        span: Span { /* position info */ }
    })
}
````

## Dependencies

- `clap = "4.0"` - For command-line argument parsing
- `rand = "0.9.2"` - For random number generation in dice rolling
- `thiserror = "2.0.12"` - For ergonomic error handling

## Development

### Running CI Tasks

```bash
# Run all CI tasks (format, check, test, clippy, build, audit)
cargo ci
```

This command runs a comprehensive set of quality assurance tasks:

- **cargo fmt**: Code formatting
- **cargo check**: Code compilation check
- **cargo test**: Unit and integration tests
- **cargo clippy**: Linting and code quality checks
- **cargo build --release**: Release build
- **cargo-audit**: Security vulnerability audit

### Running Individual Tasks

```bash
# Running tests
cargo test

# Building documentation
cargo doc --open

# Code formatting
cargo fmt

# Linting
cargo clippy

# Security audit
cargo audit
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by standard tabletop RPG dice notation
- Built with Rust's powerful parsing capabilities
