use crate::{
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxNodeChildren, SyntaxToken, T,
};
use std::{fmt, marker::PhantomData};

pub trait AstNode {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode;
}

pub trait AstToken {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxToken;
}

pub enum UnaryOp {
    Pos,
    Neg,
    BitNeg,
    Not,
}

pub enum BinaryOp {
    Or,
    And,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    In,
    NotIn,
    BitOr,
    BitXor,
    BitAnd,
    BitShiftLeft,
    BitShiftRight,
    Sub,
    Add,
    Mul,
    Mod,
    Div,
    FloorDiv,
}

pub enum Expr {
    IfExpr(IfExpr),
    UnaryExpr(UnaryExpr),
    BinaryExpr(BinaryExpr),
    TupleExpr(TupleExpr),
    LambdaExpr(LambdaExpr),
    DotExpr,
    CallExpr,
    SliceExpr,
    ListExpr,
    DictExpr,
    ListComp,
    DictComp,
    Literal,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::IfExpr(expr) => fmt::Display::fmt(expr, f),
            Expr::UnaryExpr(expr) => fmt::Display::fmt(expr, f),
            Expr::BinaryExpr(expr) => fmt::Display::fmt(expr, f),
            Expr::TupleExpr(expr) => fmt::Display::fmt(expr, f),
            Expr::LambdaExpr(expr) => fmt::Display::fmt(expr, f),
            Expr::DotExpr => todo!(),
            Expr::CallExpr => todo!(),
            Expr::SliceExpr => todo!(),
            Expr::ListExpr => todo!(),
            Expr::DictExpr => todo!(),
            Expr::ListComp => todo!(),
            Expr::DictComp => todo!(),
            Expr::Literal => todo!(),
        }
    }
}

impl AstNode for Expr {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            IF_EXPR
                | UNARY_EXPR
                | BINARY_EXPR
                | TUPLE_EXPR
                | LAMBDA_EXPR
                | DOT_EXPR
                | CALL_EXPR
                | SLICE_EXPR
                | LIST_EXPR
                | DICT_EXPR
                | LIST_COMP
                | DICT_COMP
                | LITERAL
        )
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(match syntax.kind() {
                IF_EXPR => Expr::IfExpr(IfExpr { syntax }),
                _ => todo!(),
            })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::IfExpr(expr) => expr.syntax(),
            Expr::UnaryExpr(expr) => expr.syntax(),
            Expr::BinaryExpr(expr) => expr.syntax(),
            Expr::TupleExpr(expr) => expr.syntax(),
            Expr::LambdaExpr(expr) => expr.syntax(),
            Expr::DotExpr => todo!(),
            Expr::CallExpr => todo!(),
            Expr::SliceExpr => todo!(),
            Expr::ListExpr => todo!(),
            Expr::DictExpr => todo!(),
            Expr::ListComp => todo!(),
            Expr::DictComp => todo!(),
            Expr::Literal => todo!(),
        }
    }
}

pub struct IfExpr {
    pub(crate) syntax: SyntaxNode,
}

impl IfExpr {
    pub fn condition(&self) -> Option<Expr> {
        children(self.syntax()).nth(1)
    }

    pub fn then_expr(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn else_expr(&self) -> Option<Expr> {
        children(self.syntax()).nth(2)
    }
}

impl fmt::Display for IfExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for IfExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == IF_EXPR
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub struct UnaryExpr {
    pub(crate) syntax: SyntaxNode,
}

impl UnaryExpr {
    pub fn expr(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn op_kind(&self) -> Option<UnaryOp> {
        let kind = match self.op_token()?.kind() {
            T![+] => UnaryOp::Pos,
            T![-] => UnaryOp::Neg,
            T![~] => UnaryOp::BitNeg,
            T![not] => UnaryOp::Not,
            _ => return None,
        };
        Some(kind)
    }

    pub fn op_token(&self) -> Option<SyntaxToken> {
        self.syntax().first_child_or_token()?.into_token()
    }
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for UnaryExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == UNARY_EXPR
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub struct BinaryExpr {
    pub(crate) syntax: SyntaxNode,
}

impl BinaryExpr {
    pub fn lhs(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn rhs(&self) -> Option<Expr> {
        children(self.syntax()).nth(1)
    }

    pub fn op_details(&self) -> Option<(SyntaxToken, BinaryOp)> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|el| el.into_token())
            .find_map(|token| {
                let op = match token.kind() {
                    T![or] => BinaryOp::Or,
                    T![and] => BinaryOp::Add,
                    T![==] => BinaryOp::Eq,
                    T![!=] => BinaryOp::Ne,
                    T![<] => BinaryOp::Lt,
                    T![>] => BinaryOp::Gt,
                    T![<=] => BinaryOp::Le,
                    T![>=] => BinaryOp::Ge,
                    T![in] => BinaryOp::In,
                    T![not] => BinaryOp::NotIn,
                    T![|] => BinaryOp::BitOr,
                    T![&] => BinaryOp::BitAnd,
                    T![<<] => BinaryOp::BitShiftLeft,
                    T![>>] => BinaryOp::BitShiftRight,
                    T![-] => BinaryOp::Sub,
                    T![+] => BinaryOp::Add,
                    T![*] => BinaryOp::Mul,
                    T![%] => BinaryOp::Mod,
                    T![/] => BinaryOp::Div,
                    T!["//"] => BinaryOp::FloorDiv,
                    _ => return None,
                };
                Some((token, op))
            })
    }

    pub fn op_kind(&self) -> Option<BinaryOp> {
        self.op_details().map(|d| d.1)
    }

    pub fn op_token(&self) -> Option<SyntaxToken> {
        self.op_details().map(|d| d.0)
    }
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for BinaryExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == BINARY_EXPR
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub struct TupleExpr {
    pub(crate) syntax: SyntaxNode,
}

impl TupleExpr {
    pub fn exprs(&self) -> AstChildren<Expr> {
        children(self.syntax())
    }
}

impl fmt::Display for TupleExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for TupleExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TUPLE_EXPR
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub struct LambdaExpr {
    pub(crate) syntax: SyntaxNode,
}

impl LambdaExpr {
    pub fn parameters(&self) -> Option
    // pub fn exprs(&self) -> AstChildren<Expr> {
    //     children(self.syntax())
    // }
}

impl fmt::Display for LambdaExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for LambdaExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LAMBDA_EXPR
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub struct Parameters {
    pub(crate) syntax: SyntaxNode,
}

impl Parameters {
    pub fn parameters(&self) -> AstChildren<>
}

impl fmt::Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for Parameters {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PARAMETERS
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub struct Parameter {
    pub(crate) syntax: SyntaxNode,
}

impl Parameter {
    // pub fn name
}

impl fmt::Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for Parameters {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PARAMETERS
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

pub struct Ident {
    pub(crate) syntax: SyntaxToken,
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.syntax, f)
    }
}

impl AstToken for Ident {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == IDENT
    }

    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}

#[derive(Debug, Clone)]
pub struct AstChildren<N: AstNode> {
    inner: SyntaxNodeChildren,
    _ph: PhantomData<N>,
}

impl<N: AstNode> Iterator for AstChildren<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find_map(N::cast)
    }
}

fn child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
    parent.children().find_map(N::cast)
}

fn children<N: AstNode>(parent: &SyntaxNode) -> AstChildren<N> {
    AstChildren {
        inner: parent.children(),
        _ph: PhantomData,
    }
}
