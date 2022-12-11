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

// Expression recovery
// If no expression, recover until newline or closing rparen if depth > 0.

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
        _ => or_expr(p),
    }
}

// pub(crate) fn comma_expr(p: &mut Parser) {
//     let checkpoint = p.checkpoint();
//     or_expr(p);
//     while p.at(OR) {
//         p.enter_at(checkpoint, BINARY_EXPR);
//         p.bump_any();
//         or_expr(p);
//         p.exit()
//     }
// }

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
            T![.] => {
                p.enter_at(checkpoint, DOT_EXPR);
                p.bump(T![.]);
                p.expect(T![ident]);
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
            T!['('] => {}
            _ => break,
        }
    }
}

/// Operand = identifier
///         | int | float | string | bytes
///         | ListExpr | ListComp
///         | DictExpr | DictComp
///         | '(' [Expression [',']] ')'
///         .
pub(crate) fn atom_expr(p: &mut Parser) {
    match p.current() {
        T![ident] | INT | FLOAT | STRING => p.bump_any(),
        _ => p.error("expected expression"),
    }
}
