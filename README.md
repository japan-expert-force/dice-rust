# Dice Rust

A Rust library and command-line tool for parsing dice notation expressions.

## Overview

Dice Rust is a parser for dice notation, commonly used in tabletop role-playing games. It can parse expressions like "2d6" (roll 2 six-sided dice) or "3d20" (roll 3 twenty-sided dice) and convert them into an Abstract Syntax Tree (AST) for further processing.

## Features

- **Lexical Analysis**: Tokenizes dice notation strings into meaningful tokens
- **Parsing**: Converts token streams into a structured AST
- **Error Handling**: Comprehensive error reporting with position information
- **Type Safety**: Built with Rust's type system for reliability

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

```bash
cargo run
```

This will parse the hardcoded expression "2d100" and display the resulting AST.

## Project Structure

```
src/
├── main.rs     # Command-line interface
├── lib.rs      # Library exports
├── lexer.rs    # Tokenization logic
├── parser.rs   # Parsing logic
├── ast.rs      # Abstract Syntax Tree definitions
└── error.rs    # Error handling
```

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
```

## Dependencies

- `rand = "0.9.2"` - For random number generation (future dice rolling functionality)
- `thiserror = "2.0.12"` - For ergonomic error handling

## Development

### Running Tests

```bash
cargo test
```

### Building Documentation

```bash
cargo doc --open
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by standard tabletop RPG dice notation
- Built with Rust's powerful parsing capabilities
