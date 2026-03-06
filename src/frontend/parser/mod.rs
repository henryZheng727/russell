pub mod ast;

mod parse_defn;
mod parse_expr;
mod parse_stmnt;

use crate::frontend::lexer::token::Token;
use crate::frontend::parser::ast::Defn;

pub fn parse(tokens: Vec<Token>) -> Vec<Defn> {
    unimplemented!()
}

fn skip_errors(tokens: &[Token]) -> &[Token] {
    unimplemented!()
}
