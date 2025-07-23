use crate::error::{ParseError, Position, Span};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    U32(u32),

    // Operators
    Dice, // d or D

    // End of file
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::U32(n) => write!(f, "{n}"),
            TokenKind::Dice => write!(f, "D"),
            TokenKind::Eof => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::CharIndices<'a>,
    current: Option<(usize, char)>,
    position: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.char_indices();
        let current = chars.next();

        Self {
            input,
            chars,
            current,
            position: Position::new(1, 1, 0),
        }
    }

    pub fn lex(&self) -> Result<Vec<Token>, ParseError> {
        let mut lexer = Lexer::new(self.input);
        lexer.tokenize()
    }

    fn current_char(&self) -> Option<char> {
        self.current.map(|(_, c)| c)
    }

    fn current_offset(&self) -> usize {
        self.current.map_or(self.input.len(), |(offset, _)| offset)
    }

    fn advance(&mut self) -> Option<char> {
        if let Some((_, c)) = self.current {
            if c == '\n' {
                self.position.line += 1;
                self.position.column = 1;
            } else {
                self.position.column += 1;
            }
            self.position.offset += c.len_utf8() as u32;
        }

        self.current = self.chars.next();
        self.current_char()
    }

    fn read_identifier(&mut self) -> Result<Token, ParseError> {
        let start_pos = self.position;
        let start_offset = self.current_offset();

        while let Some(c) = self.current_char() {
            if c.is_alphabetic() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let end_offset = self.current_offset();
        let text = &self.input[start_offset..end_offset];

        let kind = match text {
            "d" | "D" => TokenKind::Dice,
            _ => {
                return Err(ParseError::lexical_error(
                    Span::new(start_pos, self.position),
                    format!("Invalid identifier: {text}"),
                ));
            }
        };

        Ok(Token::new(kind, Span::new(start_pos, self.position)))
    }

    fn read_number(&mut self) -> Result<Token, ParseError> {
        let start_pos = self.position;
        let start_offset = self.current_offset();

        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let end_offset = self.current_offset();
        let text = &self.input[start_offset..end_offset];
        match text.parse::<u32>() {
            Ok(value) => Ok(Token::new(
                TokenKind::U32(value),
                Span::new(start_pos, self.position),
            )),
            Err(_) => Err(ParseError::invalid_number_literal(
                Span::new(start_pos, self.position),
                format!("Invalid number literal: {text}"),
            )),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, ParseError> {
        let start_pos = self.position;

        match self.current_char() {
            Some(c) if c.is_ascii_digit() => self.read_number(),
            Some(c) if c.is_alphabetic() => self.read_identifier(),
            Some(c) => {
                self.advance();
                Err(ParseError::lexical_error(
                    Span::new(start_pos, self.position),
                    format!("Unexpected character: {c}"),
                ))
            }
            None => Ok(Token::new(TokenKind::Eof, Span::single(start_pos))),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token.kind, TokenKind::Eof);
            tokens.push(token);

            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }
}
