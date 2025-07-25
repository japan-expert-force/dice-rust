use crate::{analyzer::SemanticAnalyzer, error::RuntimeError};
use rand::prelude::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Instruction {
    // Constants
    LdcI4(u32), // Load 32-bit integer constant

    // Local variables
    Stloc0, // Store to local variable 0
    Stloc1, // Store to local variable 1
    Stloc2, // Store to local variable 2
    Ldloc0, // Load from local variable 0
    Ldloc1, // Load from local variable 1
    Ldloc2, // Load from local variable 2

    // Stack manipulation
    Pop, // Pop value from stack
    Dup, // Duplicate top stack value

    // Arithmetic operations
    Add, // Pop two values, push sum
    Sub, // Pop two values, push difference
    Mul, // Pop two values, push product
    Div, // Pop two values, push quotient
    Rem, // Pop two values, push remainder

    // Comparison operations
    Ceq, // Compare equal
    Cgt, // Compare greater than
    Clt, // Compare less than

    // Branching
    Br(isize),      // Unconditional branch (relative offset)
    Brtrue(isize),  // Branch if true (relative offset)
    Brfalse(isize), // Branch if false (relative offset)

    // Method calls
    Call(String), // Call method
    Ret,          // Return from method

    // I/O operations
    CallWriteLine,           // Write line to console (stdout)
    CallWrite,               // Write to console (stdout)
    CallWriteStr(String),    // Write string to console (stdout)
    CallWriteLineErr,        // Write line to stderr
    CallWriteStrErr(String), // Write string to stderr

    // Random number generation
    CallRandom, // Generate random number
}

type Bytecode = Vec<Instruction>;

struct Compiler;
impl Compiler {
    pub fn compile(source: &str) -> Result<Bytecode, Box<dyn std::error::Error>> {
        let mut bytecode = Vec::<Instruction>::new();
        let mut analyzer = match SemanticAnalyzer::new(source) {
            Ok(analyzer) => analyzer,
            Err(e) => return Err(Box::new(e)),
        };
        let ast = match analyzer.analyze() {
            Ok(ast) => ast,
            Err(e) => return Err(Box::new(e)),
        };
        if let Some(stmt) = ast.statement {
            match stmt.kind {
                crate::ast::StatementKind::Expression { expr } => {
                    let crate::ast::ExpressionKind::Dice { count, faces } = expr.kind;

                    // Initialize locals: count(0), faces(1), total(2)
                    bytecode.push(Instruction::LdcI4(count)); // 0
                    bytecode.push(Instruction::Stloc0); // 1: local 0 = count

                    bytecode.push(Instruction::LdcI4(faces)); // 2
                    bytecode.push(Instruction::Stloc1); // 3: local 1 = faces

                    bytecode.push(Instruction::LdcI4(0)); // 4
                    bytecode.push(Instruction::Stloc2); // 5: local 2 = total = 0

                    // Loop start (PC = 6)
                    bytecode.push(Instruction::Ldloc0); // 6: load count
                    bytecode.push(Instruction::LdcI4(0)); // 7
                    bytecode.push(Instruction::Cgt); // 8: count > 0

                    // If count <= 0, break out of loop - we'll fix this offset later
                    let brfalse_index = bytecode.len(); // Remember index 9
                    bytecode.push(Instruction::Brfalse(0)); // 9: placeholder

                    // Generate random number
                    bytecode.push(Instruction::Ldloc1); // 10: load faces
                    bytecode.push(Instruction::CallRandom); // 11: generate random [1, faces]

                    // Print the roll result
                    bytecode.push(Instruction::Dup); // 12: duplicate for printing
                    bytecode.push(Instruction::CallWriteLine); // 13

                    // Add to total
                    bytecode.push(Instruction::Ldloc2); // 14: load total
                    bytecode.push(Instruction::Add); // 15: total + roll
                    bytecode.push(Instruction::Stloc2); // 16: store new total

                    // Decrement count
                    bytecode.push(Instruction::Ldloc0); // 17: load count
                    bytecode.push(Instruction::LdcI4(1)); // 18
                    bytecode.push(Instruction::Sub); // 19: count - 1
                    bytecode.push(Instruction::Stloc0); // 20: store new count

                    // Jump back to loop start (PC 6)
                    let br_back_offset = 6_isize - 21_isize; // From PC 21 back to PC 6
                    bytecode.push(Instruction::Br(br_back_offset)); // 21

                    // After loop: print total if original count > 1
                    let loop_exit_pc = bytecode.len(); // This is PC 22
                    if count > 1 {
                        bytecode.push(Instruction::CallWriteStrErr("Total: ".to_string())); // 22: print "Total: " to stderr
                        bytecode.push(Instruction::Ldloc2); // 23: load total
                        bytecode.push(Instruction::CallWriteLineErr); // 24: print total with newline to stderr
                    }

                    // Now fix the brfalse offset to point to the correct exit location
                    let brfalse_offset = loop_exit_pc as isize - brfalse_index as isize;
                    bytecode[brfalse_index] = Instruction::Brfalse(brfalse_offset);
                }
            }
        }
        Ok(bytecode)
    }
}

pub struct StackVm {
    stack: Vec<u32>,
    locals: [u32; 3], // Local variables 0, 1, 2
    rng: ThreadRng,
}

impl Default for StackVm {
    fn default() -> Self {
        Self::new()
    }
}

impl StackVm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            locals: [0; 3],
            rng: ThreadRng::default(),
        }
    }

    pub fn execute(&mut self, source: &str) -> Result<(), Box<dyn std::error::Error>> {
        let bytecode = Compiler::compile(source)?;
        let mut pc = 0;

        while pc < bytecode.len() {
            let instruction = &bytecode[pc];
            let jump_offset = self.execute_instruction(instruction)?;

            if jump_offset == isize::MAX {
                // Ret instruction - exit execution loop
                break;
            } else if jump_offset == 0 {
                pc += 1;
            } else {
                // Apply relative offset for branches
                let new_pc = (pc as isize) + jump_offset;
                if new_pc < 0 {
                    return Err(Box::new(RuntimeError::InvalidStackState));
                } else if new_pc >= bytecode.len() as isize {
                    // Jump beyond bytecode end - treat as program termination
                    break;
                } else {
                    pc = new_pc as usize;
                }
            }
        }

        // Program completed successfully
        Ok(())
    }

    fn execute_instruction(
        &mut self,
        instruction: &Instruction,
    ) -> Result<isize, Box<dyn std::error::Error>> {
        match instruction {
            // Constants
            Instruction::LdcI4(value) => {
                self.stack.push(*value);
            }

            // Local variables
            Instruction::Stloc0 => {
                let value = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                self.locals[0] = value;
            }
            Instruction::Stloc1 => {
                let value = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                self.locals[1] = value;
            }
            Instruction::Stloc2 => {
                let value = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                self.locals[2] = value;
            }
            Instruction::Ldloc0 => {
                self.stack.push(self.locals[0]);
            }
            Instruction::Ldloc1 => {
                self.stack.push(self.locals[1]);
            }
            Instruction::Ldloc2 => {
                self.stack.push(self.locals[2]);
            }

            // Stack manipulation
            Instruction::Pop => {
                self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
            }
            Instruction::Dup => {
                let value = self
                    .stack
                    .last()
                    .copied()
                    .ok_or(RuntimeError::InvalidStackState)?;
                self.stack.push(value);
            }

            // Arithmetic operations
            Instruction::Add => {
                let b = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                let a = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                self.stack.push(a.wrapping_add(b));
            }
            Instruction::Sub => {
                let b = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                let a = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                self.stack.push(a.wrapping_sub(b));
            }
            Instruction::Mul => {
                let b = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                let a = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                self.stack.push(a.wrapping_mul(b));
            }
            Instruction::Div => {
                let b = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                let a = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                if b == 0 {
                    return Err(Box::new(RuntimeError::InvalidStackState));
                }
                self.stack.push(a / b);
            }
            Instruction::Rem => {
                let b = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                let a = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                if b == 0 {
                    return Err(Box::new(RuntimeError::InvalidStackState));
                }
                self.stack.push(a % b);
            }

            // Comparison operations
            Instruction::Ceq => {
                let b = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                let a = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                self.stack.push(if a == b { 1 } else { 0 });
            }
            Instruction::Cgt => {
                let b = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                let a = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                self.stack.push(if a > b { 1 } else { 0 });
            }
            Instruction::Clt => {
                let b = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                let a = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                self.stack.push(if a < b { 1 } else { 0 });
            }

            // Branching
            Instruction::Br(offset) => {
                return Ok(*offset);
            }
            Instruction::Brtrue(offset) => {
                let value = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                if value != 0 {
                    return Ok(*offset);
                }
            }
            Instruction::Brfalse(offset) => {
                let value = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                if value == 0 {
                    return Ok(*offset);
                }
            }

            // I/O operations
            Instruction::CallWriteLine => {
                let value = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                println!("{value}");
            }
            Instruction::CallWrite => {
                let value = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                print!("{value}");
            }
            Instruction::CallWriteStr(s) => {
                print!("{s}");
            }
            Instruction::CallWriteLineErr => {
                let value = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                eprintln!("{value}");
            }
            Instruction::CallWriteStrErr(s) => {
                eprint!("{s}");
            }

            // Random number generation
            Instruction::CallRandom => {
                let max = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                if max == 0 {
                    self.stack.push(0);
                } else {
                    // Generate random number between 1 and max (inclusive)
                    let random_value = self.rng.random_range(1..=max);
                    self.stack.push(random_value);
                }
            }

            // Method calls and control flow
            Instruction::Call(_) => {
                return Err(Box::new(RuntimeError::InvalidStackState));
            }
            Instruction::Ret => {
                // Return from method - signal to exit the execution loop
                return Ok(isize::MAX); // Special value to indicate program end
            }
        };
        Ok(0) // No jump, continue to next instruction
    }
}
