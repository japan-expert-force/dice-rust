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
- **Limited JVM Support**: Execute simple Java .class files (basic operations only)
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
# Run with custom expression using stack VM
cargo run -- run "3d6"
cargo run -- run "1d20"
```

#### 2. JVM-Compatible VM Execution

```bash
# Execute using JVM-compatible virtual machine
cargo run -- run --jvm "2d6"
cargo run -- run --jvm "3d20" --verbose
```

#### 3. Java Class Generation and Execution

```bash
# Generate Java class file
cargo run -- compile "2d6"
cargo run -- compile "3d20" --output MyDiceClass

# Execute using built-in JVM-compatible VM
cargo run -- execute DiceRoll.class
cargo run -- execute MyDiceClass.class --verbose

# Execute using system Java (requires Java runtime)
java DiceRoll
java MyDiceClass

# Execute simple Java/Kotlin class files
javac Main.java && cargo run -- execute Main.class
kotlinc Main.kt && cargo run -- execute MainKt.class
```

### Examples

```bash
# Stack VM execution
$ cargo run -q -- run "2D100"
74
94
Total: 168

# JVM-compatible VM execution
$ cargo run -q -- run "2D100" --jvm
54
78
Total: 132

# Java class generation and built-in JVM execution
$ cargo run -q -- compile "2D100" && cargo run -q -- execute DiceRoll.class
Generated: DiceRoll.class
Run with: java DiceRoll
View bytecode with: javap -c DiceRoll.class
84
22
Total: 106

# Java class generation and system Java execution
$ cargo run -q -- compile "2D100" && java DiceRoll
Generated: DiceRoll.class
Run with: java DiceRoll
View bytecode with: javap -c DiceRoll.class
22
1
Total: 23

# Standard Java class execution
$ javac Main.java && java Main
Hello, world!

# Built-in JVM execution of Java classes
$ javac Main.java && cargo run -q -- execute Main.class
Hello, world!

# Standard Kotlin class execution
$ kotlinc Main.kt && kotlin MainKt
Hello, world!

# Built-in JVM execution of Kotlin classes
$ kotlinc Main.kt && cargo run -q -- execute MainKt.class
Hello, world!
```

## Project Structure

```
src/
├── analyzer.rs          # Semantic analysis
├── ast.rs              # Abstract Syntax Tree definitions
├── error.rs            # Error types and handling
├── lexer.rs            # Lexical analysis
├── lib.rs              # Library interface
├── main.rs             # CLI interface
├── parser.rs           # Syntax analysis
├── stack_vm.rs         # Native stack-based virtual machine
└── jvm/                # JVM-related modules
    ├── mod.rs              # JVM module exports
    ├── class_file_parser.rs    # Java class file parser
    ├── java_class_generator.rs # Java class file generation
    ├── jvm_compatible_vm.rs    # JVM-compatible virtual machine
    └── jvm_types.rs            # JVM type definitions
ci/                     # CI tooling
├── Cargo.toml          # CI tool configuration
└── src/
    └── main.rs         # CI task runner
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

3. **Java Class Generator** (`jvm/java_class_generator.rs`): Bytecode compilation
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

4. **JVM Types** (`jvm/jvm_types.rs`): JVM instruction definitions
   - Complete JVM instruction set implementation
   - Constant pool management
   - Bytecode generation and optimization

5. **Class File Parser** (`jvm/class_file_parser.rs`): Java class file parsing
   - Reads and parses .class files
   - Supports JVM-compatible execution

6. **Error Handling** (`error.rs`): Comprehensive error types
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
4. Execute using built-in JVM-compatible VM

### CI Tool

The project includes a custom CI tool in the `ci/` directory that automates quality assurance tasks:

- **Formatting**: `cargo fmt`
- **Type checking**: `cargo check`
- **Testing**: `cargo test`
- **Linting**: `cargo clippy`
- **Building**: `cargo build --release`
- **Security audit**: `cargo audit`
- **Documentation**: `cargo doc`

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

### Workspace Structure

This project uses a Cargo workspace with two members:

- **Main project** (`.`): The dice-rust library and CLI tool
- **CI tool** (`ci/`): Custom CI automation tool

## License

This project is licensed under the MIT License - see the LICENSE file for details.
