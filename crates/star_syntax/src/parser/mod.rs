use crate::{
    lexer::{Lexer, LexerReturn},
    SyntaxKind::{self, *},
    SyntaxKindSet, SyntaxNode, T,
};
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder};
use std::mem;

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

pub struct Parse {
    errors: Vec<(String, usize)>,
    green: GreenNode,
}

enum State {
    Init,
    Normal,
    Exiting,
}

pub(crate) struct Parser<'a> {
    builder: GreenNodeBuilder<'static>,
    errors: Vec<(String, usize)>,
    tokens: Vec<(SyntaxKind, usize)>,
    tokens_without_whitespace: Vec<SyntaxKind>,
    input: &'a str,
    pos: usize,        // `tokens_without_whitespace` position
    source_pos: usize, // `tokens` position
    state: State,
    text_pos: usize, // position in source file
}

impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn errors(&self) -> &[(String, usize)] {
        &self.errors
    }
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: Vec<(SyntaxKind, usize)>, input: &'a str) -> Self {
        let tokens_without_whitespace = tokens
            .iter()
            .filter_map(|(kind, _)| {
                if *kind != WHITESPACE {
                    Some(*kind)
                } else {
                    None
                }
            })
            .collect();

        Self {
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
            tokens,
            tokens_without_whitespace,
            input,
            pos: 0,
            source_pos: 0,
            state: State::Init,
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
            .cloned()
            .unwrap_or(EOF)
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
        self.do_bump(kind);
        true
    }

    fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    fn bump_any(&mut self) {
        self.do_bump(self.current());
    }

    fn do_bump(&mut self, kind: SyntaxKind) {
        // self.builder.token(kind.into(), "");
        self.pos += 1;
    }

    fn checkpoint(&mut self) -> Checkpoint {
        self.builder.checkpoint()
    }

    fn enter(&mut self) {
        match mem::replace(&mut self.state, State::Normal) {
            State::Init => unreachable!(),
            State::Normal => (),
            State::Exiting => self.builder.finish_node(),
        }
        // Consume whitespace
    }

    // Starts a new node in the GreenNodeBuilder. Consumes all whitespace until seeing the specified SyntaxKind,
    // then starts the node and consumes the SyntaxKind.
    // fn start_node(&mut self, kind: SyntaxKind, mark: SyntaxKind) {
    //     assert!(self.at(mark));
    //     self.builder.start_node(kind.into());
    //     self.bump(mark);
    //     // self.pos += 1;
    //     // self.source_consume_whitespace();
    //     // self.builder.start_node(kind.into());
    //     // self.builder.token(mark.into(), "");
    // }

    // fn start_node_any(&mut self, kind: SyntaxKind) {
    //     // self.source_consume_whitespace();
    //     self.builder.start_node(kind.into());
    // }

    // Finishes a node in the GreenNodeBuilder. Consumes all whitespace until seeing the specified SyntaxKind,
    // then consumes the SyntaxKind and finishes the node.
    // fn finish_node(&mut self, kind: SyntaxKind) {
    //     self.bump(kind);
    //     self.builder.finish_node();
    // }

    // /// Consumes whitespace in the original tokens Vec.
    // fn source_consume_whitespace(&mut self) {
    //     while self.source_at(WHITESPACE) {
    //         self.builder.token(WHITESPACE.into(), "");
    //         self.source_pos += 1;
    //         self.source_do_bump();
    //     }
    // }

    // fn source_at(&self, kind: SyntaxKind) -> bool {
    //     self.source_current() == kind
    // }

    // fn source_current(&self) -> SyntaxKind {
    //     self.tokens
    //         .get(self.source_pos)
    //         .map(|(kind, _)| *kind)
    //         .unwrap_or(EOF)
    // }

    // fn source_current_len(&self) -> usize {
    //     self.tokens
    //         .get(self.source_pos)
    //         .map(|(_, len)| *len)
    //         .unwrap_or(0)
    // }

    // fn source_bump(&mut self) {
    //     self.source_consume_whitespace();
    //     self.builder.token(self.source_current().into(), "");
    //     self.source_do_bump();
    // }

    // fn source_do_bump(&mut self) {
    //     self.text_pos += self.source_current_len();
    //     self.source_pos += 1;
    // }

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
        eprintln!("{:?}", p.current());
        statement(p);
    }
    p.builder.finish_node();
}

pub fn parse_file(input: &str) -> Parse {
    let mut errors: Vec<String> = Vec::new();

    let tokens = Lexer::from_str(input)
        .map(|LexerReturn(token, error)| {
            if let Some(error) = error {
                errors.push(error);
            }
            (token.kind, token.len)
        })
        .collect::<Vec<_>>();

    eprintln!("{:?}", tokens);

    let mut p = Parser::new(tokens, input);
    file(&mut p);

    Parse {
        errors: p.errors,
        green: p.builder.finish(),
    }
}
