use crate::frontend::parser::ast::{Binding, Type};
use crate::frontend::parser::{ParseResult, Parser};

pub(super) fn parse_type(parser: &mut Parser) -> ParseResult<Type> {
    unimplemented!()
}

pub(super) fn parse_binding(parser: &mut Parser) -> ParseResult<Binding> {
    unimplemented!()
}

pub(super) fn parse_binding_list(
    parser: &mut Parser,
) -> ParseResult<Vec<Binding>> {
    unimplemented!()
}
