use super::*;

pub(crate) const PARAMETER_START: SyntaxKindSet = SyntaxKindSet::new(&[IDENT, STAR_STAR, STAR]);

// `Parameters = Parameter {',' Parameter}.`
// test parameters
// def foo(x, y=1+2, *z, **w):
//     pass
pub(crate) fn parameters(p: &mut Parser) {
    p.enter(PARAMETERS);
    parameter(p);
    while !p.at(EOF) && !p.at(T![')']) && !p.at(T![:]) {
        if !(p.at(T![,]) && PARAMETER_START.contains(p.nth(1))) {
            break;
        }
        p.bump(T![,]);
        parameter(p);
    }
    p.exit();
}

// `Parameter = identifier | identifier '=' Test | '*' identifier | '**' identifier .`
// This function is not as strict as the rule given above. Namely, we allow parsing
// default values even for parameters with '*' or '**', with the expectation that this
// will be caught later on (e.g. during typechecking).
// test parameters_default_always_ok
// def foo(x=1, *y=1, **z=1):
//     pass
pub(crate) fn parameter(p: &mut Parser) {
    p.enter(PARAMETER);
    match p.current() {
        T![*] | T![**] => p.bump_any(),
        kind if EXPR_START.contains(kind) => (),
        _ => unreachable!(),
    }
    if p.expect(T![ident]) && p.eat(T![=]) {
        test(p, true);
    }
    p.exit();
}
