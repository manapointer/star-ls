use rowan::{Checkpoint, GreenNode, GreenNodeBuilder};

use crate::{
    SyntaxKind::{self, *},
    SyntaxKindSet, SyntaxNode, T,
};

mod expressions;
mod params;
mod statements;
mod suite;

#[cfg(test)]
mod tests;

use expressions::*;
use params::*;
use statements::*;
use suite::*;

pub(crate) struct Parse {
    errors: Vec<(String, usize)>,
    green: GreenNode,
}

pub(crate) struct Parser<'a> {
    builder: GreenNodeBuilder<'static>,
    errors: Vec<(String, usize)>,
    tokens: Vec<(SyntaxKind, usize)>,
    tokens_without_whitespace: Vec<(SyntaxKind, usize)>,
    src: &'a str,
    pos: usize,        // `tokens_without_whitespace` position
    source_pos: usize, // `tokens` position
    text_pos: usize,   // position in source file
}

impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: Vec<(SyntaxKind, usize)>, src: &'a str) -> Self {
        let tokens_without_whitespace = tokens
            .iter()
            .cloned()
            .filter(|(kind, _)| *kind != SyntaxKind::WHITESPACE)
            .collect();
        Self {
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
            tokens,
            tokens_without_whitespace,
            src,
            pos: 0,
            source_pos: 0,
            text_pos: 0,
        }
    }

    fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    fn nth(&self, n: usize) -> SyntaxKind {
        assert!(n < 2);
        self.tokens_without_whitespace
            .get(self.pos + n)
            .map(|&(kind, _)| kind)
            .unwrap_or(SyntaxKind::EOF)
    }

    fn at(&self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        self.nth(n) == kind
    }

    fn eat(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }
        self.pos += 1;
        self.source_bump();
        true
    }

    fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    fn bump_any(&mut self) {
        self.pos += 1;
        self.source_bump();
    }

    fn checkpoint(&mut self) -> Checkpoint {
        self.source_consume_whitespace();
        self.builder.checkpoint()
    }

    // Starts a new node in the GreenNodeBuilder. Consumes all whitespace until seeing the specified SyntaxKind,
    // then starts the node and consumes the SyntaxKind.
    fn start_node(&mut self, kind: SyntaxKind, mark: SyntaxKind) {
        assert!(self.at(mark));
        self.pos += 1;
        self.source_consume_whitespace();
        self.builder.token(kind.into(), "");
        self.source_pos += 1;
    }

    fn start_node_at(&mut self, kind: SyntaxKind, mark: SyntaxKind) {}

    // Finishes a node in the GreenNodeBuilder. Consumes all whitespace until seeing the specified SyntaxKind,
    // then consumes the SyntaxKind and finishes the node.
    fn finish_node(&mut self, kind: SyntaxKind) {
        self.bump(kind);
        self.builder.finish_node();
    }

    /// Consumes whitespace in the original tokens Vec.
    fn source_consume_whitespace(&mut self) {
        while self.source_at(WHITESPACE) {
            self.builder.token(WHITESPACE.into(), "");
            self.source_pos += 1;
            self.source_do_bump();
        }
    }

    fn source_at(&self, kind: SyntaxKind) -> bool {
        self.source_current() == kind
    }

    fn source_current(&self) -> SyntaxKind {
        self.tokens
            .get(self.source_pos)
            .map(|(kind, _)| *kind)
            .unwrap_or(EOF)
    }

    fn source_current_len(&self) -> usize {
        self.tokens
            .get(self.source_pos)
            .map(|(_, len)| *len)
            .unwrap_or(0)
    }

    fn source_bump(&mut self) {
        self.source_consume_whitespace();
        self.builder.token(self.source_current().into(), "");
        self.source_do_bump();
    }

    fn source_do_bump(&mut self) {
        self.text_pos += self.source_current_len();
        self.source_pos += 1;
    }

    fn error(&mut self, msg: &str) {
        self.errors.push((msg.to_string(), self.text_pos))
    }

    fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            return true;
        }
        self.error(&format!("expected {:?}", kind));
        false
    }
}

pub(crate) fn file(p: &mut Parser) {
    p.builder.start_node(FILE.into());
    while !p.at(EOF) {
        match p.current() {
            T!['\n'] => p.bump(T!['\n']),
            _ => statement(p),
        }
    }
    p.builder.finish_node();
}
