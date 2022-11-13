use super::*;

pub(crate) const ATOM_EXPR_START: SyntaxKindSet =
    SyntaxKindSet::new(&[T![ident], INT, STRING, T!['('], T!['['], T!['{']]);

pub(crate) const EXPR_START: SyntaxKindSet = ATOM_EXPR_START.union(SyntaxKindSet::new(&[
    T![if],
    T![+],
    T![-],
    T![~],
    T![not],
    T![lambda],
]));

pub(crate) fn parse_assign_or_expression(p: &mut Parser) {
    let checkpoint = p.checkpoint();
}

pub(crate) fn expression(p: &mut Parser) {
    test(p);
}

pub(crate) fn test(p: &mut Parser) {
    match p.current() {
        T![if] => todo!(),
        T![lambda] => todo!(),
        _ => {
            p.error("Expected expression");
            p.bump_any();
        }
        // T![ident] => p.bump(IDENT),
        // INT => p.bump(INT),
        // STRING => p.bump(STRING),
        // _ => p.error("Expected expression"),
    }
}
pub(crate) fn atom_expr(p: &mut Parser) {}
