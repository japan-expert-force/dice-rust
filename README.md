# Dice Rust

A Rust library and command-line tool for parsing dice notation expressions with multi-VM support and Java bytecode generation.

## Overview

Dice Rust is a comprehensive parser and execution environment for dice notation, commonly used in tabletop role-playing games. It can parse expressions like "2d6" (roll 2 six-sided dice) or "3d20" (roll 3 twenty-sided dice) and execute them on multiple virtual machine backends, including native Stack VM, JVM-compatible VM, and Java class generation.

## Features

- **Lexical Analysis**: Tokenizes dice notation strings into meaningful tokens
- **Parsing**: Converts token streams into a structured AST
- **Semantic Analysis**: Validates the AST for correctness
- **Multiple VM Backends**:
  - **Stack VM**: Native Rust stack-based virtual machine
  - **JVM-Compatible VM**: JVM bytecode-compatible execution engine
  - **Java Class Generation**: Generate executable Java .class files
- **Error Handling**: Comprehensive error reporting with position information
- **Type Safety**: Built with Rust's type system for reliability
- **Cross-Platform**: Support for multiple execution environments

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

### Command Line Interface

The tool provides three execution modes:

#### 1. Stack VM Execution (Default)

```bash
# Run with default expression (2d100)
cargo run -- run

# Run with custom expression
cargo run -- run "3d6"
cargo run -- run "1d20"
```

#### 2. JVM-Compatible VM Execution

```bash
# Execute using JVM-compatible virtual machine
cargo run -- run --jvm "2d6"
cargo run -- run --jvm "3d20"
```

#### 3. Java Class Generation

```bash
# Generate Java class file
cargo run -- java "2d6"
cargo run -- java "3d20" --output MyDiceClass

# Run the generated Java class
java DiceRoll  # or java MyDiceClass
```

### Examples

```bash
# Stack VM execution
$ cargo run -- run "2d6"
5
2
Total: 7

# JVM-compatible VM execution
$ cargo run -- run --jvm "2d6"
4
3
Total: 7

# Java class generation
$ cargo run -- java "3d6" --output GameDice
Generated: GameDice.class
Run with: java GameDice
View bytecode with: javap -c GameDice.class

$ java GameDice
2
5
1
Total: 8
```

## Project Structure

```
src/
├── analyzer.rs          # Semantic analysis
├── ast.rs              # Abstract Syntax Tree definitions
├── error.rs            # Error types and handling
├── java_generator.rs   # Java class file generation
├── jvm_bytecode.rs     # JVM bytecode definitions and compilation
├── jvm_compatible_vm.rs # JVM-compatible virtual machine
├── lexer.rs            # Lexical analysis
├── lib.rs              # Library interface
├── main.rs             # CLI interface
├── parser.rs           # Syntax analysis
└── stack_vm.rs         # Native stack-based virtual machine
```

````

## Architecture

### Virtual Machine Backends

1. **Stack VM** (`stack_vm.rs`): Native Rust implementation
   - Simple stack-based execution model
   - Optimized for direct execution
   - Minimal overhead

2. **JVM-Compatible VM** (`jvm_compatible_vm.rs`): JVM bytecode execution
   - Implements JVM specification compliance
   - Operand stack and local variables
   - Method frames and call stack management
   - Support for JVM instruction set

3. **Java Class Generator** (`java_generator.rs`): Bytecode compilation
   - Generates standard Java .class files
   - Compatible with any JVM implementation
   - Produces optimized bytecode

### Core Components

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

4. **JVM Bytecode** (`jvm_bytecode.rs`): JVM instruction definitions
   - Complete JVM instruction set implementation
   - Constant pool management
   - Bytecode generation and optimization

4. **Error Handling** (`error.rs`): Comprehensive error types
   - Position-aware error reporting
   - Detailed error messages for debugging

### Virtual Machine Execution Flow

#### Stack VM
1. Parse dice expression into AST
2. Compile AST to native VM instructions
3. Execute on simple stack-based interpreter

#### JVM-Compatible VM
1. Parse dice expression into AST
2. Compile to JVM bytecode instructions
3. Execute on JVM-compatible interpreter with method frames

#### Java Class Generation
1. Parse dice expression into AST
2. Generate complete Java class with main method
3. Output .class file compatible with standard JVM

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
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by standard tabletop RPG dice notation
- Built with Rust's powerful parsing capabilities
