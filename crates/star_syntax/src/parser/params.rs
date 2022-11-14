use super::*;

pub(crate) fn parameters(p: &mut Parser) {
    p.builder.start_node(PARAMETERS.into());
    p.bump(IDENT);
    while !p.at(EOF) && !p.at(T![')']) && !p.at(T![:]) {
        if !p.expect(T![,]) {
            break;
        }
        if !p.expect(T![ident]) {
            break;
        }
    }
    p.builder.finish_node();
}
