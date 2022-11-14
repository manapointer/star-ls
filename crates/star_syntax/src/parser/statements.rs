use super::*;

pub(crate) const SMALL_STMT_START: SyntaxKindSet =
    SyntaxKindSet::new(&[T![return], T![break], T![continue], T![pass], T![load]]);

pub(crate) fn statement(p: &mut Parser) {
    match p.current() {
        T![def] => def_stmt(p),
        kind if SMALL_STMT_START.contains(kind) => simple_stmt(p),
        _ => {
            p.error("Expected statement");
            p.bump_any();
        }
    }
}

pub(crate) fn simple_stmt(p: &mut Parser) {
    p.builder.start_node(SIMPLE_STMT.into());
    small_stmt(p);
    while p.at(T![;]) && SMALL_STMT_START.contains(p.nth(1)) {
        p.bump(T![;]);
        small_stmt(p);
    }
    p.eat(T![;]);
    if !p.at(EOF) && !p.eat(T!['\n']) {
        p.builder.start_node(ERROR.into());
        p.bump_any();
        while !p.at(EOF) && !p.at(T!['\n']) {
            p.bump_any();
        }
        p.builder.finish_node();
    }
    p.eat(T!['\n']);
    p.builder.finish_node();
}

pub(crate) fn small_stmt(p: &mut Parser) {
    match p.current() {
        T![return] => return_stmt(p),
        T![break] => break_stmt(p),
        T![continue] => continue_stmt(p),
        T![pass] => pass_stmt(p),
        T![ident] => p.bump(T![ident]),
        _ => unreachable!(),
    }
}

pub(crate) fn return_stmt(p: &mut Parser) {}

// BreakStmt = 'break'
pub(crate) fn break_stmt(p: &mut Parser) {}

// ContinueStmt = 'continue'
pub(crate) fn continue_stmt(p: &mut Parser) {}

// PassStmt = 'pass'
pub(crate) fn pass_stmt(p: &mut Parser) {}

pub(crate) fn def_stmt(p: &mut Parser) {
    p.start_node(SyntaxKind::DEF_STMT, T![def]);

    if !p.eat(T![ident]) {
        p.builder.finish_node();
        p.error("Expected function name after def");
        return;
    }

    if !p.eat(T!['(']) {
        p.builder.finish_node();
        p.error("Expected opening '(' for parameter list");
        return;
    }

    if p.at(T![ident]) {
        parameters(p);
        p.eat(T![,]);
    }

    // Try to eat the matching ')'
    if !p.eat(T![')']) {
        p.error("Expected closing ')' for parameter list");
        // We don't have the matching ')'. Start an error node and recover to the next ':'
        p.builder.start_node(ERROR.into());
        while !p.at(EOF) && !p.at(T![:]) {
            p.bump_any();
        }
        p.builder.finish_node();
    }

    let mut checked = false;

    // Try to eat the ending ':'
    if !p.at(T![:]) {
        eprintln!("current: {:?}", p.current());
        p.error("expected ':'");
        checked = true;

        // If we don't have it, recover to the next ':' or '\n'
        p.builder.start_node(ERROR.into());
        while !p.at(EOF) && !p.at(T![:]) && !p.at(T!['\n']) {
            eprintln!("bumping: {:?}", p.current());
            p.bump_any();
        }
        p.builder.finish_node();
    }

    match p.current() {
        T![:] => {
            eprintln!("colon path");
            p.bump(T![:]);
            match p.current() {
                T!['\n'] => suite(p),
                kind if SMALL_STMT_START.contains(kind) => {}
                _ => {
                    p.builder.start_node(ERROR.into());
                    while !p.at(EOF) && !p.at(T!['\n']) {
                        p.bump_any();
                    }
                    p.builder.finish_node();
                    p.eat(T!['\n']);
                    return;
                }
            }
        }
        T!['\n'] => {
            if !checked {
                eprintln!("fgfhg");
                p.error("expected ':'");
            }

            eprintln!("next: {:?}", p.nth(1));

            // If next token is INDENT, can parse suite. Otherwise, consume '\n' and finish.
            if !p.nth_at(1, INDENT) {
                p.finish_node(T!['\n']);
                return;
            }
            suite(p);
        }
        _ => {}
    }

    p.builder.finish_node();
}
