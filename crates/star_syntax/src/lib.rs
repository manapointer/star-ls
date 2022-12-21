pub mod lexer;
pub mod lines;
pub mod parser;
pub mod syntax_kind;

pub(crate) use crate::syntax_kind::*;
pub use crate::{
    parser::{parse_file, Parse},
    syntax_kind::SyntaxKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StarlarkLanguage {}

impl rowan::Language for StarlarkLanguage {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<StarlarkLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<StarlarkLanguage>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
pub type WalkEvent = rowan::WalkEvent<SyntaxElement>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub message: String,
    pub pos: usize,
}

impl Diagnostic {
    pub fn new(message: String, pos: usize) -> Diagnostic {
        Diagnostic { message, pos }
    }
}
