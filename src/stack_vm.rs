use crate::{analyzer::SemanticAnalyzer, error::RuntimeError};
use rand::prelude::*;

#[derive(Debug, Clone)]
enum Instruction {
    PushInt(u32),
    Dice,
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
                    bytecode.push(Instruction::PushInt(count));
                    bytecode.push(Instruction::PushInt(faces));
                    bytecode.push(Instruction::Dice);
                }
            }
        }
        Ok(bytecode)
    }
}

pub struct StackVm {
    stack: Vec<u32>,
    rng: ThreadRng,
}

impl StackVm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            rng: ThreadRng::default(),
        }
    }

    pub fn execute(&mut self, source: &str) -> Result<(), Box<dyn std::error::Error>> {
        let bytecode = match Compiler::compile(source) {
            Ok(bytecode) => bytecode,
            Err(e) => return Err(e),
        };
        for instruction in bytecode {
            self.execute_instruction(&instruction)?;
        }
        if self.stack.len() == 0 {
            Ok(())
        } else {
            Err(Box::new(RuntimeError::InvalidStackState))
        }
    }

    fn execute_instruction(
        &mut self,
        instruction: &Instruction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match instruction {
            Instruction::PushInt(value) => {
                self.stack.push(*value);
            }
            Instruction::Dice => {
                let faces = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                let count = self.stack.pop().ok_or(RuntimeError::InvalidStackState)?;
                let mut total = 0;
                for _ in 0..count {
                    let roll = self.rng.next_u32() % faces + 1;
                    println!("{roll}");
                    total += roll;
                }
                if count > 1 {
                    eprintln!("Total: {}", total);
                }
            }
        };
        Ok(())
    }
}
