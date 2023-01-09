use crate::{
    lexer::{Lexer, LexerReturn},
    Diagnostic,
    SyntaxKind::{self, *},
    SyntaxKindSet, SyntaxNode, T,
};
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder};
use std::mem;

mod arguments;
mod expressions;
mod parameters;
mod statements;
mod suite;

#[cfg(test)]
mod tests;

use arguments::*;
use expressions::*;
use parameters::*;
use statements::*;
use suite::*;

pub(crate) const RECOVERY_SET: SyntaxKindSet = SyntaxKindSet::new(&[T!['\n']]);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parse {
    errors: Vec<Diagnostic>,
    green: GreenNode,
}

enum State {
    Uninitialized,
    Normal,
    PendingExit,
}

pub(crate) struct Parser<'a> {
    builder: GreenNodeBuilder<'static>,
    errors: Vec<Diagnostic>,
    tokens: Vec<(SyntaxKind, usize)>,
    tokens_without_whitespace: Vec<SyntaxKind>,
    input: &'a str,
    pos: usize,        // `tokens_without_whitespace` position
    source_pos: usize, // `tokens` position
    state: State,
    input_pos: usize, // position in source file
}

impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn errors(&self) -> &[Diagnostic] {
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
            state: State::Uninitialized,
            input_pos: 0,
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

    fn error_and_recover(&mut self, target: SyntaxKindSet) {
        self.enter(ERROR);
        self.eat_until(target);
        self.exit();
    }

    fn eat_until(&mut self, target: SyntaxKindSet) {
        while self.current() != EOF && !target.contains(self.current()) {
            self.bump_any();
        }
    }

    fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    fn bump_any(&mut self) {
        self.do_bump(self.current());
    }

    fn do_bump(&mut self, kind: SyntaxKind) {
        self.pos += 1;
        self.token(kind);
    }

    fn token(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, State::Normal) {
            State::Uninitialized => unreachable!(),
            State::Normal => (),
            State::PendingExit => self.builder.finish_node(),
        }
        self.eat_whitespace();
        let (actual_kind, len) = self.tokens[self.source_pos];
        assert_eq!(kind, actual_kind);
        self.eat_token(kind, len);
    }

    fn checkpoint(&mut self) -> Checkpoint {
        match mem::replace(&mut self.state, State::Normal) {
            State::Uninitialized => unreachable!(),
            State::Normal => (),
            State::PendingExit => self.builder.finish_node(),
        }
        // Consume whitespace
        self.eat_whitespace();
        self.builder.checkpoint()
    }

    fn enter_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        match mem::replace(&mut self.state, State::Normal) {
            State::Uninitialized => unreachable!(),
            State::Normal => (),
            State::PendingExit => self.builder.finish_node(),
        }
        self.builder.start_node_at(checkpoint, kind.into())
    }

    fn enter(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, State::Normal) {
            State::Uninitialized => {
                self.builder.start_node(kind.into());
                return;
            }
            State::Normal => (),
            State::PendingExit => self.builder.finish_node(),
        }

        self.eat_whitespace();
        self.builder.start_node(kind.into());
    }

    fn exit(&mut self) {
        match mem::replace(&mut self.state, State::PendingExit) {
            State::Uninitialized => unreachable!(),
            State::Normal => (),
            State::PendingExit => self.builder.finish_node(),
        }
    }

    fn eat_whitespace(&mut self) {
        while self.source_pos < self.tokens.len() {
            let (kind, len) = self.tokens[self.source_pos];
            if !kind.is_whitespace() {
                break;
            }
            self.eat_token(kind, len)
        }
    }

    fn eat_token(&mut self, kind: SyntaxKind, len: usize) {
        self.source_pos += 1;
        self.builder.token(
            kind.into(),
            &self.input[self.input_pos..self.input_pos + len],
        );
        self.input_pos += len;
    }

    fn error_unexpected(&mut self, kind: SyntaxKind) {
        self.error(&format!("unexpected token: {:?}", kind));
    }

    fn error(&mut self, msg: &str) {
        self.errors
            .push(Diagnostic::new(msg.to_string(), self.input_pos))
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
    p.enter(FILE);
    while !p.at(EOF) {
        statement(p);
    }
    p.exit();
}

pub fn parse_file(input: &str) -> Parse {
    let mut errors: Vec<String> = Vec::new();

    let tokens = Lexer::new(input)
        .map(|LexerReturn(token, error)| {
            if let Some(error) = error {
                errors.push(error);
            }
            (token.kind, token.len)
        })
        .collect::<Vec<_>>();

    let mut p = Parser::new(tokens, input);
    file(&mut p);

    match mem::replace(&mut p.state, State::Normal) {
        State::Uninitialized | State::Normal => unreachable!(),
        State::PendingExit => {
            // Consume remaining whitespace before exiting from FILE.
            p.eat_whitespace();
            p.builder.finish_node();
        }
    }

    Parse {
        errors: p.errors,
        green: p.builder.finish(),
    }
}
