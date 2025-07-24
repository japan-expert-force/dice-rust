use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: u32,
    pub column: u32,
    pub offset: u32,
}

impl Position {
    pub fn new(line: u32, column: u32, offset: u32) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn single(pos: Position) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start.line == self.end.line {
            write!(f, "{}:{}", self.start.line, self.start.column)
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Lexical error at {span}: {message}")]
    LexicalError { span: Span, message: String },

    #[error("Syntax error at {span}: {message}")]
    SyntaxError { span: Span, message: String },

    #[error("Unexpected token at {span}: expected {expected}, found {found}")]
    UnexpectedToken {
        span: Span,
        expected: String,
        found: String,
    },

    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("Invalid number literal at {span}: {message}")]
    InvalidNumberLiteral { span: Span, message: String },
}

impl ParseError {
    pub fn lexical_error(span: Span, message: impl Into<String>) -> Self {
        Self::LexicalError {
            span,
            message: message.into(),
        }
    }

    pub fn syntax_error(span: Span, message: impl Into<String>) -> Self {
        Self::SyntaxError {
            span,
            message: message.into(),
        }
    }

    pub fn unexpected_token(
        span: Span,
        expected: impl Into<String>,
        found: impl Into<String>,
    ) -> Self {
        Self::UnexpectedToken {
            span,
            expected: expected.into(),
            found: found.into(),
        }
    }

    pub fn unexpected_end_of_input() -> Self {
        Self::UnexpectedEndOfInput
    }

    pub fn invalid_number_literal(span: Span, message: impl Into<String>) -> Self {
        Self::InvalidNumberLiteral {
            span,
            message: message.into(),
        }
    }
}

#[derive(Error, Debug)]
pub enum SemanticError {
    #[error("Empty program")]
    EmptyProgram,
    #[error("Dice count cannot be zero")]
    DiceCountZero,
    #[error("Dice faces cannot be zero")]
    DiceFacesZero,
}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Invalid stack state")]
    InvalidStackState,
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Stack overflow")]
    StackOverflow,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Invalid instruction pointer: {0}")]
    InvalidInstructionPointer(usize),
    #[error("Invalid opcode: {0}")]
    InvalidOpcode(u8),
    #[error("Call stack overflow")]
    CallStackOverflow,
    #[error("Call stack underflow")]
    CallStackUnderflow,
}
