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

pub enum ParameterKind {
    Args,
    Kwargs,
    Normal,
}

pub enum ArgumentKind {
    Args,
    Kwargs,
    Normal,
}

pub enum ListCompClause {
    ForComp,
    IfComp,
}

impl fmt::Display for ListCompClause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl AstNode for ListCompClause {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        todo!()
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
}

pub enum Stmt {
    DefStmt(DefStmt),
    IfStmt,
    ForStmt,
    SimpleStmt,
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::DefStmt(stmt) => fmt::Display::fmt(stmt, f),
            Stmt::IfStmt => todo!(),
            Stmt::ForStmt => todo!(),
            Stmt::SimpleStmt => todo!(),
        }
    }
}

impl AstNode for Stmt {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        todo!()
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
}

pub struct DefStmt {
    syntax: SyntaxNode,
}

impl DefStmt {
    pub fn name(&self) -> Option<Ident> {
        child_token(self.syntax())
    }

    pub fn parameters(&self) -> Option<Parameters> {
        child(self.syntax())
    }

    pub fn suite(&self) -> Option<Suite> {
        child(self.syntax())
    }
}

impl fmt::Display for DefStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for DefStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == DEF_STMT
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

pub struct IfStmt {
    syntax: SyntaxNode,
}

impl IfStmt {
    pub fn if_condition(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn if_branch(&self) -> Option<Suite> {
        child(self.syntax())
    }

    pub fn elif_conditions(&self) -> Vec<Expr> {
        self.syntax()
            .children_with_tokens()
            .skip_while(|el| el.as_token().map(|token| token.kind()) != Some(T![if]))
            .take_while(|el| el.as_token().map(|token| token.kind()) != Some(T![else]))
            .filter_map(|el| el.into_node())
            .filter_map(Expr::cast)
            .collect()
    }

    pub fn elif_suites(&self) -> Vec<Suite> {
        self.syntax()
            .children_with_tokens()
            .skip_while(|el| el.as_token().map(|token| token.kind()) != Some(T![if]))
            .take_while(|el| el.as_token().map(|token| token.kind()) != Some(T![else]))
            .filter_map(|el| el.into_node())
            .filter_map(Suite::cast)
            .collect()
    }

    pub fn else_condition(&self) -> Option<Expr> {
        self.syntax()
            .children_with_tokens()
            .skip_while(|el| el.as_token().map(|token| token.kind()) != Some(T![else]))
            .filter_map(|el| el.into_node())
            .find_map(Expr::cast)
    }

    pub fn else_suite(&self) -> Option<Suite> {
        self.syntax()
            .children_with_tokens()
            .skip_while(|el| el.as_token().map(|token| token.kind()) != Some(T![else]))
            .filter_map(|el| el.into_node())
            .find_map(Suite::cast)
    }

    pub fn suite(&self) -> Option<Suite> {
        child(self.syntax())
    }
}

impl fmt::Display for IfStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for IfStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == IF_STMT
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

pub enum Expr {
    IfExpr(IfExpr),
    UnaryExpr(UnaryExpr),
    BinaryExpr(BinaryExpr),
    TupleExpr(TupleExpr),
    LambdaExpr(LambdaExpr),
    DotExpr(DotExpr),
    CallExpr(CallExpr),
    SliceExpr(SliceExpr),
    ListExpr(ListExpr),
    DictExpr,
    ListComp(ListComp),
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
            Expr::DotExpr(expr) => fmt::Display::fmt(expr, f),
            Expr::CallExpr(expr) => fmt::Display::fmt(expr, f),
            Expr::SliceExpr(expr) => fmt::Display::fmt(expr, f),
            Expr::ListExpr(expr) => fmt::Display::fmt(expr, f),
            Expr::DictExpr => todo!(),
            Expr::ListComp(expr) => fmt::Display::fmt(expr, f),
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
                UNARY_EXPR => Expr::UnaryExpr(UnaryExpr { syntax }),
                BINARY_EXPR => Expr::BinaryExpr(BinaryExpr { syntax }),
                TUPLE_EXPR => Expr::TupleExpr(TupleExpr { syntax }),
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
            Expr::DotExpr(expr) => expr.syntax(),
            Expr::CallExpr(expr) => expr.syntax(),
            // Expr::SliceExpr(expr) => expr.syntax(),
            Expr::ListExpr(expr) => expr.syntax(),
            // Expr::DictExpr(expr) => expr.syntax(),
            Expr::ListComp(expr) => expr.syntax(),
            Expr::DictComp => todo!(),
            Expr::Literal => todo!(),
            _ => todo!(),
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
    pub fn parameters(&self) -> Option<Parameters> {
        child(self.syntax())
    }

    pub fn suite(&self) -> Option<Suite> {
        child(self.syntax())
    }
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

pub struct DotExpr {
    pub(crate) syntax: SyntaxNode,
}

impl DotExpr {
    pub fn expr(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn ident(&self) -> Option<Ident> {
        child_token(self.syntax())
    }
}

impl fmt::Display for DotExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for DotExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == DOT_EXPR
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

pub struct CallExpr {
    pub(crate) syntax: SyntaxNode,
}

impl CallExpr {
    pub fn expr(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn arguments(&self) -> Option<Arguments> {
        child(self.syntax())
    }
}

impl fmt::Display for CallExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for CallExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == CALL_EXPR
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

pub struct SliceExpr {
    pub(crate) syntax: SyntaxNode,
}

impl SliceExpr {
    pub fn expr(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn start(&self) -> Option<Expr> {
        children(self.syntax()).nth(1)
    }

    pub fn end(&self) -> Option<Expr> {
        children(self.syntax()).nth(2)
    }

    pub fn step(&self) -> Option<Expr> {
        children(self.syntax()).nth(3)
    }
}

impl fmt::Display for SliceExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for SliceExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SLICE_EXPR
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

pub struct ListExpr {
    pub(crate) syntax: SyntaxNode,
}

impl ListExpr {
    pub fn elements(&self) -> Vec<Expr> {
        children(self.syntax()).collect()
    }
}

impl fmt::Display for ListExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for ListExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LIST_EXPR
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

pub struct ListComp {
    pub(crate) syntax: SyntaxNode,
}

impl ListComp {
    pub fn expr(&self) -> Option<Expr> {
        child(self.syntax())
    }
}

impl fmt::Display for ListComp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for ListComp {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LIST_COMP
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

pub struct ListCompFor {
    pub(crate) syntax: SyntaxNode,
}

impl ListCompFor {
    pub fn expr(&self) -> Option<Expr> {
        child(self.syntax())
    }
}

impl fmt::Display for ListCompFor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for ListCompFor {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LIST_COMP
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
    pub fn parameters(&self) -> AstChildren<Parameter> {
        children(self.syntax())
    }
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
    pub fn kind(&self) -> ParameterKind {
        match self
            .syntax()
            .first_child_or_token()
            .and_then(|el| el.into_token())
            .map(|tok| tok.kind())
        {
            Some(T![*]) => ParameterKind::Args,
            Some(T![**]) => ParameterKind::Kwargs,
            _ => ParameterKind::Normal,
        }
    }

    pub fn name(&self) -> Option<Ident> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|el| el.into_token())
            .find_map(Ident::cast)
    }

    pub fn default(&self) -> Option<Expr> {
        child(self.syntax())
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for Parameter {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PARAMETER
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

pub struct Arguments {
    pub(crate) syntax: SyntaxNode,
}

impl Arguments {
    pub fn arguments(&self) -> AstChildren<Argument> {
        children(self.syntax())
    }
}

impl fmt::Display for Arguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for Arguments {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ARGUMENTS
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

pub struct Argument {
    pub(crate) syntax: SyntaxNode,
}

impl Argument {
    pub fn kind(&self) -> ArgumentKind {
        match self
            .syntax()
            .first_child_or_token()
            .and_then(|el| el.into_token())
            .map(|tok| tok.kind())
        {
            Some(T![*]) => ArgumentKind::Args,
            Some(T![**]) => ArgumentKind::Kwargs,
            _ => ArgumentKind::Normal,
        }
    }

    pub fn name(&self) -> Option<Ident> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|el| el.into_token())
            .find_map(Ident::cast)
    }

    pub fn value(&self) -> Option<Expr> {
        child(self.syntax())
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for Argument {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ARGUMENT
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

pub struct Suite {
    pub(crate) syntax: SyntaxNode,
}

impl Suite {
    pub fn statements(&self) -> Vec<Stmt> {
        children(self.syntax()).collect()
    }

    // pub fn simple_stmt(&self) -> Option<>
}

impl fmt::Display for Suite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for Suite {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SUITE
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

fn child_token<T: AstToken>(parent: &SyntaxNode) -> Option<T> {
    parent
        .children_with_tokens()
        .filter_map(|el| el.into_token())
        .find_map(T::cast)
}
