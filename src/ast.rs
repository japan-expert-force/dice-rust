use crate::error::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statement: Option<Statement>,
}
impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    pub fn new() -> Self {
        Self { statement: None }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    Expression { expr: Expression },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    Dice { count: u32, faces: u32 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Dice,
}

impl Statement {
    pub fn expr_stmt(expr: Expression, span: Span) -> Self {
        Self {
            kind: StatementKind::Expression { expr },
            span,
        }
    }
}

impl Expression {
    pub fn dice(count: u32, faces: u32, span: Span) -> Self {
        Self {
            kind: ExpressionKind::Dice { count, faces },
            span,
        }
    }
}
