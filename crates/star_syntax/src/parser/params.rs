use super::*;

pub(crate) const PARAMETER_START: SyntaxKindSet = SyntaxKindSet::new(&[IDENT, STAR_STAR, STAR]);

/// `Parameters = Parameter {',' Parameter}.`
pub(crate) fn parameters(p: &mut Parser) {
    p.enter(PARAMETERS);
    parameter(p);

    while !p.at(EOF) && !p.at(T![')']) && !p.at(T![:]) {
        if !(p.at(T![,]) && p.nth_at(1, T![ident])) {
            break;
        }
        p.bump(T![,]);
        parameter(p);
    }

    p.exit();
}

/// `Parameter = identifier | identifier '=' Test | '*' identifier | '**' identifier .`
/// This function is not as strict as the rule given above. Namely, we allow parsing
/// default values even for parameters with '*' or '**', with the expectation that this
/// will be caught later on (e.g. during typechecking).
pub(crate) fn parameter(p: &mut Parser) {
    p.enter(PARAMETER);

    match p.current() {
        T![*] | T![**] => p.bump_any(),
        T![ident] => (),
        _ => unreachable!(),
    }

    if !p.expect(T![ident]) {
        return;
    }

    if p.eat(T![=]) {}

    p.exit();
}
