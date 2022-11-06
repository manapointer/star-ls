use super::*;

pub(crate) fn statement(p: &mut Parser) {
    match p.current() {
        T![def] => def_statement(p),
        _ => todo!(),
    }
}

pub(crate) fn def_statement(p: &mut Parser) {
    p.start_node(SyntaxKind::DEF_STMT, T![def]);
    if p.eat(T![ident]) {
        p.builder.finish_node();
        p.error("Expected function name after def");
        return;
    }
    if p.at(T!['(']) {
        p.builder.finish_node();
        p.error("Expected opening '(' for parameter list");
        return;
    }
    if p.at(T![,]) {
        p.bump(T![,]);
    }
    if !p.at(T![')']) {
        // Recover to COLON.
    } else if !p.at('\n') {
    }
    // if p.at(T)
}
