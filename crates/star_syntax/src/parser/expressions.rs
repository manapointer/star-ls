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

// test tuple_expr
// 1, 2
// ()
// (x)
// (x, y + 1)
// (x, y + 1, z + 2)
// (x, y, z,)
pub(crate) fn expression_or_tuple(p: &mut Parser, parens: bool, force_expr_list: bool) -> usize {
    let checkpoint = p.checkpoint();
    let mut did_checkpoint = false;
    if parens {
        p.bump(T!['(']);
        if p.eat(T![')']) {
            p.enter_at(checkpoint, TUPLE_EXPR);
            p.exit();
            return 0;
        }
    }
    if !test(p, true) {
        p.error_unexpected(p.current());
        p.error_and_recover(RECOVERY_SET);
        return 0;
    }
    let mut len = 1;
    while p.at(T![,]) && EXPR_START.contains(p.nth(1)) {
        if !did_checkpoint && !force_expr_list {
            did_checkpoint = true;
            p.enter_at(checkpoint, TUPLE_EXPR);
        }
        p.bump(T![,]);
        if !test(p, true) {
            p.error_unexpected(p.current());
            p.error_and_recover(RECOVERY_SET);
        }
        len += 1;
    }

    // test_err tuple_expr_invalid
    // (1,,)
    // 1, 1,
    // (1, 2 def
    if parens {
        p.eat(T![,]);
        let res = p.expect(T![')']);
        if did_checkpoint {
            p.exit();
        }
        if !res {
            p.error_and_recover(RECOVERY_SET);
        }
        return len;
    } else if did_checkpoint {
        p.exit();
    }
    len
}

// test test_expr
// 1
// 1 if 2 else 3
// 1 if 2 else 3 if 4 else 5
pub(crate) fn test(p: &mut Parser, allow_if: bool) -> bool {
    match p.current() {
        T![lambda] => todo!(),
        _ => {
            let checkpoint = p.checkpoint();
            if !or_expr(p) {
                return false;
            }
            if !allow_if {
                return true;
            }
            if !p.eat(T![if]) {
                return true;
            }
            p.enter_at(checkpoint, IF_EXPR);
            or_expr(p);
            if !p.expect(T![else]) {
                p.exit();
                return true;
            }
            test(p, true);
            p.exit();
        }
    }
    true
}

// test or_expr
// 1 or 2
// 1 and 2 or 3 and 4
// 1 == 2 and 3 == 4
// 1 | 2 == 3 | 4
pub(crate) fn or_expr(p: &mut Parser) -> bool {
    let checkpoint = p.checkpoint();
    if and_expr(p) {
        while p.at(T![or]) {
            p.enter_at(checkpoint, BINARY_EXPR);
            p.bump_any();
            and_expr(p);
            p.exit()
        }
        true
    } else {
        false
    }
}

pub(crate) fn and_expr(p: &mut Parser) -> bool {
    let checkpoint = p.checkpoint();
    if eq_expr(p) {
        while p.at(T![and]) {
            p.enter_at(checkpoint, BINARY_EXPR);
            p.bump_any();
            eq_expr(p);
            p.exit();
        }
        true
    } else {
        false
    }
}

pub(crate) fn eq_expr(p: &mut Parser) -> bool {
    let checkpoint = p.checkpoint();
    if bitwise_or_expr(p) {
        while matches!(
            p.current(),
            T![==] | T![!=] | T![<] | T![>] | T![<=] | T![>=] | T![in]
        ) {
            p.enter_at(checkpoint, BINARY_EXPR);
            p.bump_any();
            bitwise_or_expr(p);
            p.exit();
        }
        true
    } else {
        false
    }
}

pub(crate) fn bitwise_or_expr(p: &mut Parser) -> bool {
    let checkpoint = p.checkpoint();
    if bitwise_xor_expr(p) {
        while p.at(T![|]) {
            p.enter_at(checkpoint, BINARY_EXPR);
            p.bump_any();
            bitwise_xor_expr(p);
            p.exit();
        }
        true
    } else {
        false
    }
}

pub(crate) fn bitwise_xor_expr(p: &mut Parser) -> bool {
    let checkpoint = p.checkpoint();
    if bitwise_and_expr(p) {
        while p.at(T![^]) {
            p.enter_at(checkpoint, BINARY_EXPR);
            p.bump_any();
            bitwise_and_expr(p);
            p.exit();
        }
        true
    } else {
        false
    }
}

pub(crate) fn bitwise_and_expr(p: &mut Parser) -> bool {
    let checkpoint = p.checkpoint();
    if bitwise_shift_expr(p) {
        while p.at(T![&]) {
            p.enter_at(checkpoint, BINARY_EXPR);
            p.bump_any();
            bitwise_shift_expr(p);
            p.exit();
        }
        true
    } else {
        false
    }
}

pub(crate) fn bitwise_shift_expr(p: &mut Parser) -> bool {
    let checkpoint = p.checkpoint();
    if add_expr(p) {
        while matches!(p.current(), T![<<] | T![>>]) {
            p.enter_at(checkpoint, BINARY_EXPR);
            p.bump_any();
            add_expr(p);
            p.exit();
        }
        true
    } else {
        false
    }
}

pub(crate) fn add_expr(p: &mut Parser) -> bool {
    let checkpoint = p.checkpoint();
    if mul_expr(p) {
        while matches!(p.current(), T![+] | T![-]) {
            p.enter_at(checkpoint, BINARY_EXPR);
            p.bump_any();
            mul_expr(p);
            p.exit();
        }
        true
    } else {
        false
    }
}

pub(crate) fn mul_expr(p: &mut Parser) -> bool {
    let checkpoint = p.checkpoint();
    if primary_expr(p) {
        while matches!(p.current(), T![*] | T![%] | T![/] | T!["//"]) {
            p.enter_at(checkpoint, BINARY_EXPR);
            p.bump_any();
            primary_expr(p);
            p.exit();
        }
        true
    } else {
        false
    }
}

pub(crate) fn primary_expr(p: &mut Parser) -> bool {
    let checkpoint = p.checkpoint();
    if atom_expr(p) {
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

                    // test_err call_expr_recover
                    // foo(1, 2, def 123)
                    let res = p.expect(T![')']);
                    p.exit();
                    if !res {
                        p.error_and_recover(RECOVERY_SET);
                    }
                }
                T!['['] => {
                    p.enter_at(checkpoint, SLICE_EXPR);
                    p.bump(T!['[']);
                    match p.current() {
                        T![:] => (),
                        kind if EXPR_START.contains(kind) => {
                            test(p, true);
                            match p.current() {
                                T![']'] => {
                                    p.eat(T![']']);
                                    p.exit();
                                    break true;
                                }
                                T![:] => (),
                                _ => {
                                    p.error("expected ':' or ']'");
                                    // TODO: Recover to closing brace?
                                    p.exit();
                                    break true;
                                }
                            }
                        }
                        _ => {
                            p.error("expected ':' or expression");
                            // TODO: Recover to closing brace?
                            p.exit();
                            break true;
                        }
                    }

                    // Parse slice.
                    p.bump(T![:]);

                    // if EXPR_START.contains(p.current()) {
                    //     test(p, true);
                    // }

                    // if p.eat(T![:]) && EXPR_START.contains(p.current()) {
                    //     test(p, true);
                    // }

                    // if !p.expect(T![']']) {
                    //     p.exit();
                    // }
                }
                _ => break true,
            }
        }
    } else {
        false
    }
}

// Operand = identifier
//         | int | float | string | bytes
//         | ListExpr | ListComp
//         | DictExpr | DictComp
//         | '(' [Expression [',']] ')'
//        .
pub(crate) fn atom_expr(p: &mut Parser) -> bool {
    match p.current() {
        T![ident] | INT | FLOAT | STRING => literal(p),
        T!['('] => {
            expression_or_tuple(p, /* parens */ true, /* force_expr_list */ false);
        }
        T!['['] => list_expr_or_comp(p),
        _ => {
            p.error("expected expression");
            return false;
        }
    }
    true
}

pub(crate) fn literal(p: &mut Parser) {
    p.enter(LITERAL);
    p.bump_any();
    p.exit();
}

pub(crate) fn list_expr_or_comp(p: &mut Parser) {
    let checkpoint = p.checkpoint();
    p.bump(T!['[']);
    match p.current() {
        T![']'] => {
            p.enter_at(checkpoint, LIST_EXPR);
            p.bump(T![']']);
            p.exit();
        }
        _ => {
            let len =
                expression_or_tuple(p, /* parens */ false, /* force_expr_list */ true);
            // If only one 'test' was parsed, and the next token is 'for', then we have a list comprehension.
            if len == 1 && p.at(T![for]) {
                // test list_comp
                // [x for x in y]
                // [x for x in y if x]
                // [(x, y) for x in a for y in b if x == y]
                p.enter_at(checkpoint, LIST_COMP);
                loop {
                    match p.current() {
                        T![for] => {
                            p.enter(LIST_COMP_FOR);
                            p.bump(T![for]);
                            loop_variables(p);
                            if !p.expect(T![in]) {
                                p.exit();
                                break;
                            }

                            test(p, false);
                            p.exit();
                        }
                        T![if] => {
                            p.enter(LIST_COMP_IF);
                            p.bump(T![if]);
                            test(p, false);
                            p.exit();
                        }
                        _ => break,
                    }
                }
            } else {
                // test list_expr
                // []
                // [1]
                // [1, 2]
                // [1, 2, 3]
                // [1, 2, 3,]
                p.enter_at(checkpoint, LIST_EXPR);
                p.eat(T![,]);
            }
            let is_closed = p.expect(T![']']);
            p.exit();
            if !is_closed {
                p.error_and_recover(RECOVERY_SET);
            }
        }
    }
}

pub(crate) fn loop_variables(p: &mut Parser) {
    p.enter(LOOP_VARIABLES);
    primary_expr(p);
    while p.at(T![,]) {
        p.bump(T![,]);
        if !ATOM_EXPR_START.contains(p.current()) {
            break;
        }
        primary_expr(p);
    }
    p.exit();
}
