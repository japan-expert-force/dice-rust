use crate::ast::{ExpressionKind, Program, StatementKind};
use crate::error::{ParseError, SemanticError};
use crate::parser::Parser;

pub struct SemanticAnalyzer {
    ast: Program,
}

impl SemanticAnalyzer {
    pub fn new(source: &str) -> Result<Self, ParseError> {
        let mut parser = Parser::new(source)?;
        let ast = parser.parse()?;
        Ok(Self { ast })
    }

    pub fn analyze(&mut self) -> Result<Program, SemanticError> {
        let statement = self
            .ast
            .statement
            .as_ref()
            .ok_or(SemanticError::EmptyProgram)?;
        let expression = match &statement.kind {
            StatementKind::Expression { expr } => expr,
        };
        match &expression.kind {
            ExpressionKind::Dice { count, faces } => {
                if *count == 0 {
                    return Err(SemanticError::DiceCountZero);
                }
                if *faces == 0 {
                    return Err(SemanticError::DiceFacesZero);
                }
            }
        };
        Ok(self.ast.clone())
    }
}
