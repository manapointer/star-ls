use super::*;

pub(crate) const SMALL_STMT_START: SyntaxKindSet =
    SyntaxKindSet::new(&[T![return], T![break], T![continue], T![pass], T![load]])
        .union(ATOM_EXPR_START);

pub(crate) fn statement(p: &mut Parser) {
    match p.current() {
        T![def] => def_stmt(p),
        kind if SMALL_STMT_START.contains(kind) => simple_stmt(p),
        T!['\n'] => p.bump(T!['\n']),
        _ => {
            p.error("expected statement");
            p.error_and_recover(RECOVERY_SET);
        }
    }
}

pub(crate) fn simple_stmt(p: &mut Parser) {
    p.enter(SIMPLE_STMT);
    small_stmt(p);
    while p.at(T![;]) && SMALL_STMT_START.contains(p.nth(1)) {
        p.bump(T![;]);
        small_stmt(p);
    }
    p.eat(T![;]);
    if !p.at(EOF) && !p.at(T!['\n']) {
        p.error_unexpected(p.current());
        p.error_and_recover(RECOVERY_SET);
    }
    p.eat(T!['\n']);
    p.exit();
}

pub(crate) fn small_stmt(p: &mut Parser) {
    match p.current() {
        T![return] => return_stmt(p),
        T![break] => break_stmt(p),
        T![continue] => continue_stmt(p),
        T![pass] => pass_stmt(p),
        kind if EXPR_START.contains(kind) => expr_or_assign_stmt(p),
        _ => {
            p.error_unexpected(p.current());
            p.bump_any();
        }
    }
}

// test assign_stmt
// x = 1
// x, y = 1, 2
// x, y = (1, 2)
// (x, y) = 1, 2
pub(crate) fn expr_or_assign_stmt(p: &mut Parser) {
    let checkpoint = p.checkpoint();
    expression_or_tuple(p, /* parens */ false, /* force_expr_list */ false);
    if matches!(
        p.current(),
        T![=]
            | T![+=]
            | T![-=]
            | T![*=]
            | T![/=]
            | T!["//="]
            | T![%=]
            | T![&=]
            | T![|=]
            | T![^=]
            | T![<<=]
            | T![>>=]
    ) {
        p.enter_at(checkpoint, ASSIGN_STMT);
        p.bump_any();
        expression_or_tuple(p, /*parens */ false, /* force_expr_list */ false);
        p.exit();
    }
}

// test return_stmt
// return
// return 1
// return 1, 2
// return (1, 2)
pub(crate) fn return_stmt(p: &mut Parser) {
    p.enter(RETURN_STMT);
    p.bump(T![return]);
    if EXPR_START.contains(p.current()) {
        expression_or_tuple(p, /* parens */ false, /* force_expr_list */ false);
    }
    p.exit();
}

// BreakStmt = 'break'
// test break_stmt
// break
pub(crate) fn break_stmt(p: &mut Parser) {
    p.enter(BREAK_STMT);
    p.bump(T![break]);
    p.exit();
}

// ContinueStmt = 'continue'
// test continue_stmt
// continue
pub(crate) fn continue_stmt(p: &mut Parser) {
    p.enter(CONTINUE_STMT);
    p.bump(T![continue]);
    p.exit();
}

// PassStmt = 'pass'
// test pass_stmt
// pass
pub(crate) fn pass_stmt(p: &mut Parser) {
    p.enter(PASS_STMT);
    p.bump(T![pass]);
    p.exit();
}

// test def_stmt
// def hello():
//     pass
pub(crate) fn def_stmt(p: &mut Parser) {
    p.enter(DEF_STMT);
    p.eat(T![def]);

    // test_err def_stmt_missing_function_name
    // def
    if !p.eat(T![ident]) {
        p.exit();
        p.error("Expected function name after def");
        return;
    }

    // test_err def_stmt_expected_opening_paren
    // def hello
    if !p.eat(T!['(']) {
        p.exit();
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
        p.enter(ERROR);
        while !p.at(EOF) && !p.at(T![:]) {
            p.bump_any();
        }
        p.exit();
    }

    let mut checked = false;

    // Check if we are at the ending ':'
    if !p.at(T![:]) {
        p.error("Expected ':'");
        checked = true;

        // If we don't have it, recover to the next ':' or '\n'
        p.enter(ERROR);
        while !p.at(EOF) && !p.at(T![:]) && !p.at(T!['\n']) {
            p.bump_any();
        }
        p.exit();
    }

    match p.current() {
        T![:] => {
            p.bump(T![:]);
            match p.current() {
                T!['\n'] => suite(p),
                kind if SMALL_STMT_START.contains(kind) => suite(p),
                _ => {
                    p.enter(ERROR);
                    while !p.at(EOF) && !p.at(T!['\n']) {
                        p.bump_any();
                    }
                    p.exit();
                    p.eat(T!['\n']);
                    return;
                }
            }
        }
        T!['\n'] => {
            if !checked {
                p.error("expected ':'");
            }

            // If next token is INDENT, can parse suite. Otherwise, consume '\n' and finish.
            if !p.nth_at(1, INDENT) {
                p.bump(T!['\n']);
                p.exit();
                return;
            }
            suite(p);
        }
        _ => {}
    }

    p.exit();
}
