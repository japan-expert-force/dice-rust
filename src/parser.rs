use crate::ast::{BinaryOperator, Expression, Program, Statement};
use crate::error::{ParseError, Position, Span};
use crate::lexer::{Lexer, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(source: &str) -> Result<Self, ParseError> {
        let tokens = match Lexer::new(source).lex() {
            Ok(tokens) => tokens,
            Err(e) => return Err(e),
        };
        Ok(Self { tokens, current: 0 })
    }

    fn current_token(&self) -> Token {
        self.tokens
            .get(self.current)
            .cloned()
            .unwrap_or_else(|| Token::new(TokenKind::Eof, Span::single(Position::new(1, 1, 0))))
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current_token().kind, TokenKind::Eof)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens
            .get(self.current - 1)
            .cloned()
            .unwrap_or_else(|| Token::new(TokenKind::Eof, Span::single(Position::new(1, 1, 0))))
    }

    fn token_to_binary_operator(&self, token: &TokenKind) -> Option<BinaryOperator> {
        match token {
            TokenKind::Dice => Some(BinaryOperator::Dice),
            _ => None,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = Program::new();

        let stmt = self.parse_statement()?;
        program.statement = Some(stmt);

        if !self.is_at_end() {
            return Err(ParseError::unexpected_end_of_input());
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match &self.current_token().kind {
            TokenKind::U32(_) => self.parse_expression_statement(),
            _ => Err(ParseError::syntax_error(
                self.current_token().span.clone(),
                "Expected a statement".to_string(),
            )),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let start_span = self.current_token().span.clone();
        let count = if let TokenKind::U32(count) = &self.current_token().kind {
            *count
        } else {
            return Err(ParseError::unexpected_token(
                self.current_token().span.clone(),
                "u32",
                format!("{:?}", self.current_token().kind),
            ));
        };
        self.advance();
        let operator_token = self.advance();
        self.token_to_binary_operator(&operator_token.kind);
        let faces = if let TokenKind::U32(faces) = &self.current_token().kind {
            *faces
        } else {
            return Err(ParseError::unexpected_token(
                self.current_token().span.clone(),
                "u32",
                format!("{:?}", self.current_token().kind),
            ));
        };
        self.advance();
        let end_span = self.current_token().span.clone();

        Ok(Statement::expr_stmt(
            Expression::dice(count, faces, Span::new(start_span.start, end_span.end)),
            Span::new(start_span.start, end_span.end),
        ))
    }
}
