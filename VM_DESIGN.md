# Multi-VM Dice Language Execution Environment Design Document

## Overview

This project implements diverse virtual machine backends for parsing and executing dice notation. The educational multi-VM design allows learning the characteristics and advantages of different execution models.

## Supported VM Types

### 1. Stack VM - Native Stack-based Virtual Machine

- **Purpose**: Simple and high-performance execution environment
- **Features**: Custom stack-based VM written in Rust
- **Use Cases**: Educational purposes and high-performance execution

### 2. JVM-Compatible VM - JVM-Compatible Virtual Machine

- **Purpose**: Execution environment compliant with JVM bytecode specification
- **Features**: Reproduces JVM operand stack and local variables
- **Use Cases**: Ensuring compatibility with JVM ecosystem

### 3. Java Class Generator - Java Class File Generation

- **Purpose**: Generation of standard Java class files
- **Features**: Outputs .class files executable on any JVM implementation
- **Use Cases**: Reuse and distribution in Java environments

## Core Components

### Stack VM - Native Implementation

#### 1. Basic Instruction Set

```rust
enum Instruction {
    PushInt(u32),  // Push integer value to stack
    Dice,          // Execute dice roll
}
```

#### 2. VM Structure

```rust
pub struct StackVm {
    stack: Vec<u32>,     // Data stack
    rng: ThreadRng,      // Random number generator
}
```

### JVM-Compatible VM - JVM Specification Compliant Implementation

#### 1. JVM Value Type System

```rust
pub enum JvmValue {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Reference(Option<usize>),
    ReturnAddress(usize),
}
```

#### 2. Method Frame

```rust
pub struct MethodFrame {
    locals: Vec<JvmValue>,        // Local variable array
    operand_stack: Vec<JvmValue>, // Operand stack
    constant_pool: ConstantPool,  // Constant pool
    pc: usize,                    // Program counter
    bytecode: Vec<JvmInstruction>, // Bytecode
}
```

#### 3. JVM Bytecode Instructions

```rust
pub enum JvmInstruction {
    // Constant loading
    Iconst0, Iconst1, Iconst2, Iconst3, Iconst4, Iconst5,
    IconstM1,
    Bipush(i8), Sipush(i16),
    Ldc(u16),

    // Stack operations
    Pop, Pop2, Dup, DupX1, DupX2, Swap,

    // Arithmetic operations
    Iadd, Isub, Imul, Idiv, Irem,
    Fadd, Fsub, Fmul, Fdiv, Frem,

    // Branching and jumping
    Ifeq(u16), Ifne(u16), Iflt(u16), Ifge(u16),
    Goto(u16),

    // Method invocation
    Invokestatic(u16), Invokevirtual(u16),

    // Return values
    Return, Ireturn, Freturn,
}
```

### Java Class Generator - Bytecode Generation

#### 1. Constant Pool Management

```rust
pub struct ConstantPool {
    entries: Vec<ConstantPoolEntry>,
    string_cache: HashMap<String, u16>,
    int_cache: HashMap<i32, u16>,
    float_cache: HashMap<String, u16>,
}
```

#### 2. Class File Generation

- **Magic Number**: 0xCAFEBABE
- **Version**: Java 8 (52.0)
- **Constant Pool**: Management of string and numeric constants
- **Methods**: Automatic generation of main method
- **Bytecode**: Instruction sequence compliant with JVM specification

## Execution Flow Comparison

### Stack VM Execution Model

```rust
// Input: "2d6"
// 1. Parse → AST
// 2. Bytecode generation
vec![
    Instruction::PushInt(2),  // Number of dice
    Instruction::PushInt(6),  // Number of faces
    Instruction::Dice,        // Execute dice roll
]
// 3. Execution
```

### JVM-Compatible VM Execution Model

```rust
// Input: "2d6"
// 1. Parse → AST
// 2. JVM bytecode generation
vec![
    JvmInstruction::Iconst2,     // Load constant 2
    JvmInstruction::Bipush(6),   // Load constant 6
    JvmInstruction::Invokestatic(method_ref), // Call dice roll method
]
// 3. Execute with JVM specification compliance
```

### Java Class Generation Model

```java
// Input: "2d6"
// 1. Parse → AST
// 2. Java class generation
public class DiceRoll {
    public static void main(String[] args) {
        rollDice(2, 6);
    }

    private static void rollDice(int count, int faces) {
        // Implemented in bytecode
    }
}
// 3. Output .class file
```

## Random Number Generation Implementation

Unified high-quality random number generation implemented across all VMs:

```rust
// Cryptographically secure random generation using ThreadRng
fn roll_dice(&mut self, count: u32, faces: u32) -> Vec<u32> {
    (0..count)
        .map(|_| self.rng.next_u32() % faces + 1)
        .collect()
}
```

## Usage Examples and Benchmarks

### 1. Stack VM - High-Speed Execution

```bash
$ cargo run -- run "3d6"
# Execution time: ~1ms
# Memory usage: Minimal
4
2
5
Total: 11
```

### 2. JVM-Compatible VM - Specification Compliant

```bash
$ cargo run -- run --jvm "3d6"
# Execution time: ~3ms
# Memory usage: Moderate
3
6
1
Total: 10
```

### 3. Java Class - Distributable

```bash
$ cargo run -- java "3d6" --output GameDice
$ java GameDice
# Execution time: JVM startup cost + ~1ms
# Distribution: Executable on any JVM environment
2
4
6
Total: 12
```

## CLI Interface

```bash
# Display help
cargo run -- --help

# Stack VM execution (default)
cargo run -- run "2d20"

# JVM-Compatible VM execution
cargo run -- run --jvm "4d6"

# Java class generation
cargo run -- java "1d100" --output LootRoll
```

## Design Advantages

### 1. Educational Value

- **Diverse VM Implementations**: Comparative learning of different execution models
- **Progressive Understanding**: From simple StackVM to full JVM specification
- **Practical Output**: Generation of actually usable .class files

### 2. Practicality

- **High Performance**: High-speed execution with native StackVM
- **Compatibility**: Integration with existing systems through JVM specification compliance
- **Distributability**: Wide execution environment support with standard Java class files

### 3. Extensibility

- **Modular Design**: Easy addition of new VM backends
- **Unified Interface**: Multiple target generation from common AST
- **Plugin Architecture**: Independent development and maintenance of each VM

## Future Extension Possibilities

### VM Feature Extensions

1. **WebAssembly Backend**: Browser execution support
2. **LLVM Backend**: Native code generation
3. **GPU Parallel Execution**: Parallelization of large-scale dice rolls

### Language Feature Extensions

1. **Variables and Scope**: Expression of more complex expressions
2. **Control Structures**: Addition of if statements and loops
3. **Function Definition**: Support for user-defined functions

### Output Format Extensions

1. **TypeScript/JavaScript**: Usage in web environments
2. **Python Bytecode**: Integration with Python ecosystem
3. **C/C++ Code Generation**: Support for embedded environments

This implementation is a comprehensive project that enables wide learning and application from basic virtual machine design concepts to practical language processing systems.
