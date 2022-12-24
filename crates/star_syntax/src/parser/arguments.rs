use super::*;

pub(crate) const ARGUMENT_START: SyntaxKindSet =
    EXPR_START.union(SyntaxKindSet::new(&[T![**], T![*]]));

pub(crate) fn arguments(p: &mut Parser) {
    p.enter(ARGUMENTS);
    argument(p);

    while p.at(T![,]) && ARGUMENT_START.contains(p.nth(1)) {
        p.bump(T![,]);
        argument(p);
    }

    p.exit();
}

pub(crate) fn argument(p: &mut Parser) {
    p.enter(ARGUMENT);

    match p.current() {
        T![*] | T![**] => p.bump_any(),
        kind if EXPR_START.contains(kind) => (),
        _ => unreachable!(),
    }

    if p.at(T![ident]) && p.nth_at(1, T![=]) {
        p.bump(T![ident]);
        p.bump(T![=]);
        test(p);
    } else {
        test(p);
    }

    p.exit();
}
