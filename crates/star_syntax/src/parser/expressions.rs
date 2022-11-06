use super::*;

pub(crate) fn parse_assign_or_expression(p: &mut Parser) {
    let checkpoint = p.checkpoint();
}

pub(crate) fn parse_expression(p: &mut Parser) {
    parse_base_expression(p);
}

pub(crate) fn parse_base_expression(p: &mut Parser) {
    match p.current() {
        T![ident] => p.bump(IDENT),
        INT => p.bump(INT),
        STRING => p.bump(STRING),
        _ => p.error("Expected expression"),
    }
}
