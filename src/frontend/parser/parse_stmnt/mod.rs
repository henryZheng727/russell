use crate::frontend::lexer::token::{Token, TokenKind};
use crate::frontend::parser::ast::Stmnt;
use crate::frontend::parser::parse_expr::parse_expr;
use crate::frontend::parser::parse_type::parse_type;
use crate::frontend::parser::{ParseResult, Parser};

pub(super) fn parse_stmnt(parser: &mut Parser) -> ParseResult<Stmnt> {
    match parser.peek() {
        Token::Let => parse_let(parser),
        Token::Read => parse_read(parser),
        Token::Echo => parse_echo(parser),
        Token::Return => parse_return(parser),
        _ => unimplemented!(), // TODO improve error handling here
    }
}

fn parse_let(parser: &mut Parser) -> ParseResult<Stmnt> {
    parser.expect(TokenKind::Let)?;
    let id = match parser.expect(TokenKind::Id)? {
        Token::Id(str) => str,
        _ => unreachable!(),
    };
    parser.expect(TokenKind::Eq)?;
    let expr = parse_expr(parser)?;
    Ok(Stmnt::Let(id, expr))
}

fn parse_read(parser: &mut Parser) -> ParseResult<Stmnt> {
    parser.expect(TokenKind::Read)?;
    let read_type = parse_type(parser)?;
    let id = match parser.expect(TokenKind::Id)? {
        Token::Id(str) => str,
        _ => unreachable!(),
    };
    Ok(Stmnt::Read(read_type, id))
}

fn parse_echo(parser: &mut Parser) -> ParseResult<Stmnt> {
    parser.expect(TokenKind::Echo)?;
    let echo_type = parse_type(parser)?;
    let expr = parse_expr(parser)?;
    Ok(Stmnt::Echo(echo_type, expr))
}

fn parse_return(parser: &mut Parser) -> ParseResult<Stmnt> {
    parser.expect(TokenKind::Return)?;
    let expr = parse_expr(parser)?;
    Ok(Stmnt::Return(expr))
}
