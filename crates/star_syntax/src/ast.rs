use crate::{
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxNodeChildren, SyntaxToken, T,
};
use std::{fmt, marker::PhantomData};

macro_rules! def_ast_node {
    ($name:ident, $kinds:pat) => {
        pub struct $name {
            syntax: SyntaxNode,
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(self.syntax(), f)
            }
        }

        impl AstNode for $name {
            fn can_cast(kind: SyntaxKind) -> bool {
                matches!(kind, $kinds)
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
    };
}

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

pub enum CompClause {
    ForComp(CompFor),
    IfComp(CompIf),
}

impl fmt::Display for CompClause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompClause::ForComp(comp) => fmt::Display::fmt(comp, f),
            CompClause::IfComp(comp) => fmt::Display::fmt(comp, f),
        }
    }
}

impl AstNode for CompClause {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, LIST_COMP_IF | LIST_COMP_FOR)
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(match syntax.kind() {
                LIST_COMP_FOR => CompClause::ForComp(CompFor { syntax }),
                LIST_COMP_IF => CompClause::IfComp(CompIf { syntax }),
                _ => unreachable!(),
            })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            CompClause::ForComp(clause) => clause.syntax(),
            CompClause::IfComp(clause) => clause.syntax(),
        }
    }
}

pub enum Stmt {
    DefStmt(DefStmt),
    IfStmt(IfStmt),
    ForStmt(ForStmt),
    SimpleStmt(SimpleStmt),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::DefStmt(stmt) => fmt::Display::fmt(stmt, f),
            Stmt::IfStmt(stmt) => fmt::Display::fmt(stmt, f),
            Stmt::ForStmt(stmt) => fmt::Display::fmt(stmt, f),
            Stmt::SimpleStmt(stmt) => fmt::Display::fmt(stmt, f),
        }
    }
}

impl AstNode for Stmt {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(kind, DEF_STMT | IF_STMT | FOR_STMT | SIMPLE_STMT)
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(match syntax.kind() {
                DEF_STMT => Stmt::DefStmt(DefStmt { syntax }),
                IF_STMT => Stmt::IfStmt(IfStmt { syntax }),
                FOR_STMT => Stmt::ForStmt(ForStmt { syntax }),
                SIMPLE_STMT => Stmt::SimpleStmt(SimpleStmt { syntax }),
                _ => unreachable!(),
            })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Stmt::DefStmt(stmt) => stmt.syntax(),
            Stmt::IfStmt(stmt) => stmt.syntax(),
            Stmt::ForStmt(stmt) => stmt.syntax(),
            Stmt::SimpleStmt(stmt) => stmt.syntax(),
        }
    }
}

def_ast_node!(DefStmt, DEF_STMT);
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

def_ast_node!(IfStmt, IF_STMT);
impl IfStmt {
    pub fn if_condition(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn if_suite(&self) -> Option<Suite> {
        child(self.syntax())
    }

    pub fn elif_conditions(&self) -> Vec<Expr> {
        self.syntax()
            .children_with_tokens()
            .skip_while(|el| el.as_token().map(SyntaxToken::kind) != Some(T![if]))
            .take_while(|el| el.as_token().map(SyntaxToken::kind) != Some(T![else]))
            .filter_map(|el| el.into_node())
            .filter_map(Expr::cast)
            .collect()
    }

    pub fn elif_suites(&self) -> Vec<Suite> {
        self.syntax()
            .children_with_tokens()
            .skip_while(|el| el.as_token().map(SyntaxToken::kind) != Some(T![if]))
            .take_while(|el| el.as_token().map(SyntaxToken::kind) != Some(T![else]))
            .filter_map(|el| el.into_node())
            .filter_map(Suite::cast)
            .collect()
    }

    pub fn else_condition(&self) -> Option<Expr> {
        child_after_token(self.syntax(), T![else])
    }

    pub fn else_suite(&self) -> Option<Suite> {
        child_after_token(self.syntax(), T![else])
    }
}

def_ast_node!(ForStmt, FOR_STMT);
impl ForStmt {
    pub fn loop_variables(&self) -> Vec<Expr> {
        children_until_token(self.syntax(), T![in]).collect()
    }

    pub fn expr(&self) -> Option<Expr> {
        child_after_token(self.syntax(), T![in])
    }

    pub fn suite(&self) -> Option<Suite> {
        child(self.syntax())
    }
}

def_ast_node!(SimpleStmt, SIMPLE_STMT);
impl SimpleStmt {
    pub fn statements(&self) -> Vec<SmallStmt> {
        children(self.syntax()).collect()
    }
}

pub enum SmallStmt {
    ReturnStmt(ReturnStmt),
    BreakStmt(BreakStmt),
    ContinueStmt(ContinueStmt),
    PassStmt(PassStmt),
    AssignStmt(AssignStmt),
    ExprStmt(Expr),
    LoadStmt(LoadStmt),
}

impl fmt::Display for SmallStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmallStmt::ReturnStmt(stmt) => fmt::Display::fmt(stmt, f),
            SmallStmt::BreakStmt(stmt) => fmt::Display::fmt(stmt, f),
            SmallStmt::ContinueStmt(stmt) => fmt::Display::fmt(stmt, f),
            SmallStmt::PassStmt(stmt) => fmt::Display::fmt(stmt, f),
            SmallStmt::AssignStmt(stmt) => fmt::Display::fmt(stmt, f),
            SmallStmt::ExprStmt(expr) => fmt::Display::fmt(expr, f),
            SmallStmt::LoadStmt(stmt) => fmt::Display::fmt(stmt, f),
        }
    }
}

impl AstNode for SmallStmt {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            RETURN_STMT
                | BREAK_STMT
                | CONTINUE_STMT
                | PASS_STMT
                | ASSIGN_STMT
                | LOAD_STMT

                // Include expression types
                | IF_EXPR
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
        )
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            Some(match syntax.kind() {
                RETURN_STMT => SmallStmt::ReturnStmt(ReturnStmt { syntax }),
                BREAK_STMT => SmallStmt::ReturnStmt(ReturnStmt { syntax }),
                CONTINUE_STMT => SmallStmt::ContinueStmt(ContinueStmt { syntax }),
                PASS_STMT => SmallStmt::PassStmt(PassStmt { syntax }),
                ASSIGN_STMT => SmallStmt::AssignStmt(AssignStmt { syntax }),
                LOAD_STMT => SmallStmt::LoadStmt(LoadStmt { syntax }),
                IF_EXPR | UNARY_EXPR | BINARY_EXPR | TUPLE_EXPR | LAMBDA_EXPR | DOT_EXPR
                | CALL_EXPR | SLICE_EXPR | LIST_EXPR | DICT_EXPR | LIST_COMP | DICT_COMP => {
                    SmallStmt::ExprStmt(Expr::cast(syntax).unwrap())
                }
                _ => unreachable!(),
            })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            SmallStmt::ReturnStmt(stmt) => stmt.syntax(),
            SmallStmt::BreakStmt(stmt) => stmt.syntax(),
            SmallStmt::ContinueStmt(stmt) => stmt.syntax(),
            SmallStmt::PassStmt(stmt) => stmt.syntax(),
            SmallStmt::AssignStmt(stmt) => stmt.syntax(),
            SmallStmt::ExprStmt(expr) => expr.syntax(),
            SmallStmt::LoadStmt(stmt) => stmt.syntax(),
        }
    }
}

def_ast_node!(ReturnStmt, RETURN_STMT);
impl ReturnStmt {
    pub fn expr(&self) -> Option<Expr> {
        child(self.syntax())
    }
}

def_ast_node!(BreakStmt, BREAK_STMT);
impl BreakStmt {}

def_ast_node!(ContinueStmt, CONTINUE_STMT);
impl ContinueStmt {}

def_ast_node!(PassStmt, PASS_STMT);
impl PassStmt {}

def_ast_node!(AssignStmt, ASSIGN_STMT);
impl AssignStmt {
    pub fn lhs(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn rhs(&self) -> Option<Expr> {
        children(self.syntax()).nth(1)
    }
}

def_ast_node!(LoadStmt, LOAD_STMT);
impl LoadStmt {
    pub fn lhs(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn rhs(&self) -> Option<Expr> {
        children(self.syntax()).nth(1)
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
    DictExpr(DictExpr),
    ListComp(ListComp),
    DictComp(DictComp),
    Literal(Literal),
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
            Expr::DictExpr(expr) => fmt::Display::fmt(expr, f),
            Expr::ListComp(expr) => fmt::Display::fmt(expr, f),
            Expr::DictComp(expr) => fmt::Display::fmt(expr, f),
            Expr::Literal(expr) => fmt::Display::fmt(expr, f),
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
                LAMBDA_EXPR => Expr::LambdaExpr(LambdaExpr { syntax }),
                DOT_EXPR => Expr::DotExpr(DotExpr { syntax }),
                CALL_EXPR => Expr::CallExpr(CallExpr { syntax }),
                SLICE_EXPR => Expr::SliceExpr(SliceExpr { syntax }),
                LIST_EXPR => Expr::ListExpr(ListExpr { syntax }),
                LIST_COMP => Expr::ListComp(ListComp { syntax }),
                DICT_EXPR => Expr::DictExpr(DictExpr { syntax }),
                DICT_COMP => Expr::DictComp(DictComp { syntax }),
                // LITERAL => Expr::Literal(Literal { syntax }),
                _ => unreachable!(),
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
            Expr::SliceExpr(expr) => expr.syntax(),
            Expr::ListExpr(expr) => expr.syntax(),
            Expr::DictExpr(expr) => expr.syntax(),
            Expr::ListComp(expr) => expr.syntax(),
            Expr::DictComp(expr) => expr.syntax(),
            Expr::Literal(expr) => expr.syntax(),
        }
    }
}

def_ast_node!(IfExpr, IF_EXPR);
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

def_ast_node!(UnaryExpr, UNARY_EXPR);
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

def_ast_node!(BinaryExpr, BINARY_EXPR);
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

def_ast_node!(TupleExpr, TUPLE_EXPR);
impl TupleExpr {
    pub fn exprs(&self) -> AstChildren<Expr> {
        children(self.syntax())
    }
}

def_ast_node!(LambdaExpr, LAMBDA_EXPR);
impl LambdaExpr {
    pub fn parameters(&self) -> Option<Parameters> {
        child(self.syntax())
    }

    pub fn suite(&self) -> Option<Suite> {
        child(self.syntax())
    }
}

def_ast_node!(DotExpr, DOT_EXPR);
impl DotExpr {
    pub fn expr(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn ident(&self) -> Option<Ident> {
        child_token(self.syntax())
    }
}

def_ast_node!(CallExpr, CALL_EXPR);
impl CallExpr {
    pub fn expr(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn arguments(&self) -> Option<Arguments> {
        child(self.syntax())
    }
}

def_ast_node!(SliceExpr, SLICE_EXPR);
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

def_ast_node!(ListExpr, LIST_EXPR);
impl ListExpr {
    pub fn elements(&self) -> Vec<Expr> {
        children(self.syntax()).collect()
    }
}

def_ast_node!(ListComp, LIST_COMP);
impl ListComp {
    pub fn expr(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn comp_clauses(&self) -> Vec<CompClause> {
        children(self.syntax()).collect()
    }
}

def_ast_node!(DictExpr, DICT_EXPR);
impl DictExpr {
    pub fn entries(&self) -> Option<Entries> {
        child(self.syntax())
    }
}

def_ast_node!(DictComp, DICT_COMP);
impl DictComp {
    pub fn expr(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn comp_clauses(&self) -> Vec<CompClause> {
        children(self.syntax()).collect()
    }
}

pub enum LiteralKind {
    Ident(Ident),
    Int(Int),
    Float(Float),
    String(String),
}

pub struct Literal {
    syntax: SyntaxNode,
}

impl Literal {
    pub fn token(&self) -> SyntaxToken {
        self.syntax()
            .children_with_tokens()
            .find(|el| !el.kind().is_whitespace())
            .and_then(|el| el.into_token())
            .unwrap()
    }

    pub fn kind(&self) -> LiteralKind {
        let token = self.token();

        if let Some(token) = Int::cast(token.clone()) {
            return LiteralKind::Int(token);
        }

        if let Some(token) = Float::cast(token.clone()) {
            return LiteralKind::Float(token);
        }

        if let Some(token) = String::cast(token) {
            return LiteralKind::String(token);
        }

        unreachable!()
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.syntax(), f)
    }
}

impl AstNode for Literal {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LITERAL
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

def_ast_node!(CompFor, LIST_COMP_FOR);
impl CompFor {
    pub fn loop_variables(&self) -> Vec<Expr> {
        children_until_token(self.syntax(), T![in]).collect()
    }

    pub fn expr(&self) -> Option<Expr> {
        child_after_token(self.syntax(), T![in])
    }
}

def_ast_node!(CompIf, LIST_COMP_IF);
impl CompIf {
    pub fn expr(&self) -> Option<Expr> {
        child(self.syntax())
    }
}

def_ast_node!(Entries, ENTRIES);
impl Entries {
    pub fn entries(&self) -> Vec<Entries> {
        children(self.syntax()).collect()
    }
}

def_ast_node!(Entry, ENTRY);
impl Entry {
    pub fn key(&self) -> Option<Expr> {
        child(self.syntax())
    }

    pub fn value(&self) -> Option<Expr> {
        child_after_token(self.syntax(), T![:])
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

pub struct Int {
    pub(crate) syntax: SyntaxToken,
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.syntax, f)
    }
}

impl AstToken for Int {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == INT
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

pub struct Float {
    pub(crate) syntax: SyntaxToken,
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.syntax, f)
    }
}

impl AstToken for Float {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FLOAT
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

pub struct String {
    pub(crate) syntax: SyntaxToken,
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.syntax, f)
    }
}

impl AstToken for String {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == STRING
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

fn children_until_token<N: AstNode>(
    parent: &SyntaxNode,
    kind: SyntaxKind,
) -> impl Iterator<Item = N> {
    parent
        .children_with_tokens()
        .take_while(move |el| el.as_token().map(SyntaxToken::kind) != Some(kind))
        .filter_map(|el| el.into_node())
        .filter_map(N::cast)
}

fn children_after_token<N: AstNode>(
    parent: &SyntaxNode,
    kind: SyntaxKind,
) -> impl Iterator<Item = N> {
    parent
        .children_with_tokens()
        .skip_while(move |el| el.as_token().map(SyntaxToken::kind) != Some(kind))
        .filter_map(|el| el.into_node())
        .filter_map(N::cast)
}

fn child_after_token<N: AstNode>(parent: &SyntaxNode, kind: SyntaxKind) -> Option<N> {
    children_after_token(parent, kind).next()
}

fn child_token<T: AstToken>(parent: &SyntaxNode) -> Option<T> {
    parent
        .children_with_tokens()
        .filter_map(|el| el.into_token())
        .find_map(T::cast)
}
