use super::*;

pub(crate) fn arguments(p: &mut Parser) {
    p.enter(ARGUMENTS);
}

pub(crate) fn argument(p: &mut Parser) {
    p.enter(ARGUMENT);

    match p.current() {
        T![*] | T![**] => p.bump_any(),
        kind if EXPR_START.contains(kind) => (),
        _ => unreachable!(),
    }

    // if !p.expect(T![])
}
