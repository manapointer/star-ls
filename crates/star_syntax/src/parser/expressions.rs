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

// test tuple_exr
// 1, 2
// ()
// (x)
// (x, y + 1)
// (x, y + 1, z + 2)
pub(crate) fn expression_or_tuple(p: &mut Parser, parens: bool) {
    let checkpoint = p.checkpoint();
    let mut did_checkpoint = false;
    if parens {
        p.bump(T!['(']);
        if p.eat(T![')']) {
            p.enter_at(checkpoint, TUPLE_EXPR);
            p.exit();
            return;
        }
    }
    test(p);
    while p.at(T![,]) && EXPR_START.contains(p.nth(1)) {
        if !did_checkpoint {
            did_checkpoint = true;
            p.enter_at(checkpoint, TUPLE_EXPR);
        }
        p.bump(T![,]);
        test(p);
    }
    if parens && !p.expect(T![')']) {
        // TODO: Recover to closing paren
    }
    if did_checkpoint {
        p.exit();
    }
}

// test test_expr
// 1
// 1 if 2 else 3
// 1 if 2 else 3 if 4 else 5
pub(crate) fn test(p: &mut Parser) {
    match p.current() {
        T![lambda] => todo!(),
        _ => {
            let checkpoint = p.checkpoint();
            or_expr(p);
            if !p.eat(T![if]) {
                // TODO: Recover
                return;
            }
            p.enter_at(checkpoint, IF_EXPR);
            or_expr(p);
            if !p.expect(T![else]) {
                p.exit();
                // TODO: Recover
                return;
            }
            test(p);
            p.exit();
        }
    }
}

// test or_expr
// 1 or 2
// 1 and 2 or 3 and 4
// 1 == 2 and 3 == 4
// 1 | 2 == 3 | 4
pub(crate) fn or_expr(p: &mut Parser) {
    let checkpoint = p.checkpoint();
    and_expr(p);
    while p.at(T![or]) {
        p.enter_at(checkpoint, BINARY_EXPR);
        p.bump_any();
        and_expr(p);
        p.exit()
    }
}

pub(crate) fn and_expr(p: &mut Parser) {
    let checkpoint = p.checkpoint();
    eq_expr(p);
    while p.at(T![and]) {
        p.enter_at(checkpoint, BINARY_EXPR);
        p.bump_any();
        eq_expr(p);
        p.exit();
    }
}

pub(crate) fn eq_expr(p: &mut Parser) {
    let checkpoint = p.checkpoint();
    bitwise_or_expr(p);
    while matches!(
        p.current(),
        T![==] | T![!=] | T![<] | T![>] | T![<=] | T![>=] | T![in]
    ) {
        p.enter_at(checkpoint, BINARY_EXPR);
        p.bump_any();
        bitwise_or_expr(p);
        p.exit();
    }
}

pub(crate) fn bitwise_or_expr(p: &mut Parser) {
    let checkpoint = p.checkpoint();
    bitwise_xor_expr(p);
    while p.at(T![|]) {
        p.enter_at(checkpoint, BINARY_EXPR);
        p.bump_any();
        bitwise_xor_expr(p);
        p.exit();
    }
}

pub(crate) fn bitwise_xor_expr(p: &mut Parser) {
    let checkpoint = p.checkpoint();
    bitwise_and_expr(p);
    while p.at(T![^]) {
        p.enter_at(checkpoint, BINARY_EXPR);
        p.bump_any();
        bitwise_and_expr(p);
        p.exit();
    }
}

pub(crate) fn bitwise_and_expr(p: &mut Parser) {
    let checkpoint = p.checkpoint();
    bitwise_shift_expr(p);
    while p.at(T![&]) {
        p.enter_at(checkpoint, BINARY_EXPR);
        p.bump_any();
        bitwise_shift_expr(p);
        p.exit();
    }
}

pub(crate) fn bitwise_shift_expr(p: &mut Parser) {
    let checkpoint = p.checkpoint();
    add_expr(p);
    while matches!(p.current(), T![<<] | T![>>]) {
        p.enter_at(checkpoint, BINARY_EXPR);
        p.bump_any();
        add_expr(p);
        p.exit();
    }
}

pub(crate) fn add_expr(p: &mut Parser) {
    let checkpoint = p.checkpoint();
    mul_expr(p);
    while matches!(p.current(), T![+] | T![-]) {
        p.enter_at(checkpoint, BINARY_EXPR);
        p.bump_any();
        mul_expr(p);
        p.exit();
    }
}

pub(crate) fn mul_expr(p: &mut Parser) {
    let checkpoint = p.checkpoint();
    primary_expr(p);
    while matches!(p.current(), T![*] | T![%] | T![/] | T!["//"]) {
        p.enter_at(checkpoint, BINARY_EXPR);
        p.bump_any();
        primary_expr(p);
        p.exit();
    }
}

pub(crate) fn primary_expr(p: &mut Parser) {
    let checkpoint = p.checkpoint();
    atom_expr(p);
    loop {
        match p.current() {
            // test dot_expr
            // a.b
            // a.b.c
            T![.] => {
                p.enter_at(checkpoint, DOT_EXPR);
                p.bump(T![.]);

                // test_err dot_expr_no_ident
                // a.
                p.expect(T![ident]);
                p.exit();
            }

            // test call_expr
            // foo()
            // foo(1)
            // foo(1, a=1+2, *b, **c)
            T!['('] => {
                p.enter_at(checkpoint, CALL_EXPR);
                p.bump(T!['(']);
                if ARGUMENT_START.contains(p.current()) {
                    arguments(p);
                    p.eat(T![,]);
                }
                // TODO: Recover strategy
                p.expect(T![')']);
                p.exit();
            }
            T!['['] => {
                p.enter_at(checkpoint, SLICE_EXPR);
                p.bump(T!['[']);
                match p.current() {
                    T![:] => (),
                    kind if EXPR_START.contains(kind) => {
                        test(p);
                        match p.current() {
                            T![']'] => {
                                p.eat(T![']']);
                                p.exit();
                                return;
                            }
                            T![:] => (),
                            _ => {
                                p.error("expected ':' or ']'");
                                // TODO: Recover to closing brace?
                                p.exit();
                                return;
                            }
                        }
                    }
                    _ => {
                        p.error("expected ':' or expression");
                        // TODO: Recover to closing brace?
                        p.exit();
                        return;
                    }
                }

                // Parse slice.
                p.bump(T![:]);

                if EXPR_START.contains(p.current()) {
                    test(p);
                }

                if p.eat(T![:]) && EXPR_START.contains(p.current()) {
                    test(p);
                }

                if !p.expect(T![:]) {
                    p.error("expected closing ']'");
                    // TODO: Recover to closing brace?
                    p.exit();
                }
            }
            _ => break,
        }
    }
}

// Operand = identifier
//         | int | float | string | bytes
//         | ListExpr | ListComp
//         | DictExpr | DictComp
//         | '(' [Expression [',']] ')'
//        .
pub(crate) fn atom_expr(p: &mut Parser) {
    match p.current() {
        T![ident] | INT | FLOAT | STRING => p.bump_any(),
        T!['('] => expression_or_tuple(p, true),
        _ => p.error("expected expression"),
    }
}
