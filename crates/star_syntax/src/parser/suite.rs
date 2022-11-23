use super::*;

/// spec: `Suite = [newline indent {Statement} outdent] | SimpleStmt .`
pub(crate) fn suite(p: &mut Parser) {
    p.enter(SUITE);
    match p.current() {
        T!['\n'] => {
            p.bump(T!['\n']);

            if p.eat(INDENT) {
                while !p.at(EOF) && !p.at(OUTDENT) {
                    statement(p);
                }
                p.eat(OUTDENT);
            } else {
                p.error("Expected an indented block");
            }
        }
        kind if SMALL_STMT_START.contains(kind) => simple_stmt(p),
        _ => unreachable!(),
    }
    p.exit();
}
