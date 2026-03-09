pub mod ast;

mod parse_defn;
mod parse_expr;
mod parse_stmnt;
mod parse_type;

use crate::frontend::lexer::token::{Token, TokenKind};
use crate::frontend::parser::ast::Defn;
use crate::frontend::parser::parse_defn::parse_defn;

pub fn parse(tokens: Vec<Token>) -> Vec<Defn> {
    let mut parser = Parser::new(tokens);
    let mut defns = Vec::new();

    while parser.peek().kind() != TokenKind::EoF {
        match parse_defn(&mut parser) {
            Ok(defn) => defns.push(defn),
            Err(err) => unimplemented!(), // TODO - handle errors
        }
    }

    return defns;
}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub offset: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, offset: 0 }
    }

    pub fn expect(&mut self, expected: TokenKind) -> ParseResult<()> {
        if self.peek().kind() == expected {
            self.advance();
            Ok(())
        } else {
            ParseError::new(expected, self.peek().clone())
        }
    }

    pub fn advance(&mut self) {
        self.offset += 1;
    }

    pub fn peek(&mut self) -> &Token {
        &self.tokens[self.offset]
    }
}

pub struct ParseError {
    expected: TokenKind,
    actual: Token,
}

impl ParseError {
    pub fn new<A>(expected: TokenKind, actual: Token) -> ParseResult<A> {
        Err(ParseError { expected, actual })
    }
}

pub type ParseResult<A> = Result<A, ParseError>;
