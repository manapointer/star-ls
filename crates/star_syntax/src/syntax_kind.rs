#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[repr(u16)]
pub enum SyntaxKind {
    EOF,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    SLASH_SLASH,
    MOD,
    STAR_STAR,
    TILDE,
    AND,
    OR,
    XOR,
    LT_LT,
    GT_GT,
    DOT,
    COMMA,
    EQ,
    SEMICOLON,
    COLON,
    L_PAREN,
    R_PAREN,
    L_BRACK,
    R_BRACK,
    L_BRACE,
    R_BRACE,
    LT,
    GT,
    GT_EQ,
    LT_EQ,
    EQ_EQ,
    BANG_EQ,
    PLUS_EQ,
    MINUS_EQ,
    STAR_EQ,
    SLASH_EQ,
    SLASH_SLASH_EQ,
    MOD_EQ,
    AND_EQ,
    OR_EQ,
    XOR_EQ,
    LT_LT_EQ,
    GT_GT_EQ,
    INT,
    FLOAT,
    STRING,
    AND_KW,
    BREAK_KW,
    CONTINUE_KW,
    DEF_KW,
    ELIF_KW,
    ELSE_KW,
    FOR_KW,
    IF_KW,
    IN_KW,
    LAMBDA_KW,
    LOAD_KW,
    NOT_KW,
    OR_KW,
    PASS_KW,
    RETURN_KW,
    AS_KW,
    ASSERT_KW,
    CLASS_KW,
    DEL_KW,
    EXECPT_KW,
    FINALLY_KW,
    FROM_KW,
    GLOBAL_KW,
    IMPORT_KW,
    IS_KW,
    NONLOCAL_KW,
    RAISE_KW,
    TRY_KW,
    WHILE_KW,
    WITH_KW,
    YIELD_KW,
    IDENT,
    INDENT,
    OUTDENT,
    WHITESPACE,
    COMMENT,
    NEWLINE,
    ERROR_TOKEN,
    ERROR,
    DEF_STMT,
    IF_STMT,
    FOR_STMT,
    SIMPLE_STMT,
    RETURN_STMT,
    BREAK_STMT,
    CONTINUE_STMT,
    PASS_STMT,
    ASSIGN_STMT,
    EXPR_STMT,
    LOAD_STMT,
    EXPR,
    IF_EXPR,
    PRIMARY_EXPR,
    UNARY_EXPR,
    BINARY_EXPR,
    TUPLE_EXPR,
    LAMBDA_EXPR,
    DOT_EXPR,
    CALL_EXPR,
    SLICE_EXPR,
    LIST_EXPR,
    DICT_EXPR,
    LIST_COMP,
    LIST_COMP_FOR,
    LIST_COMP_IF,
    DICT_COMP,
    ARGUMENTS,
    ARGUMENT,
    PARAMETERS,
    PARAMETER,
    ENTRIES,
    ENTRY,
    COMP_CLAUSE,
    SUITE,
    LOOP_VARIABLES,
    LITERAL,
    FILE,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SyntaxKindSet(u128);

impl SyntaxKindSet {
    pub(crate) const EMPTY: SyntaxKindSet = SyntaxKindSet(0);

    pub(crate) const fn new(kinds: &[SyntaxKind]) -> SyntaxKindSet {
        let mut inner = 0;
        let mut i = 0;
        while i < kinds.len() {
            inner |= 1 << kinds[i] as u16;
            i += 1;
        }
        SyntaxKindSet(inner)
    }

    pub(crate) const fn contains(&self, kind: SyntaxKind) -> bool {
        self.0 & 1 << kind as usize > 0
    }

    pub(crate) const fn union(&self, other: SyntaxKindSet) -> SyntaxKindSet {
        SyntaxKindSet(self.0 | other.0)
    }
}

impl SyntaxKind {
    pub fn is_whitespace(self: SyntaxKind) -> bool {
        matches!(self, Self::WHITESPACE | Self::COMMENT)
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[macro_export]
macro_rules! T {
    [;] => { $ crate :: SyntaxKind :: SEMICOLON } ; [:] => { $ crate :: SyntaxKind :: COLON } ; [-] => { $ crate :: SyntaxKind :: MINUS } ; [*] => { $ crate :: SyntaxKind :: STAR } ; [**] => { $ crate :: SyntaxKind :: STAR_STAR } ; [/] => { $ crate :: SyntaxKind :: SLASH } ; [/] => { $ crate :: SyntaxKind :: SLASH } ; [=] => { $ crate :: SyntaxKind :: EQ } ; [+=] => { $ crate :: SyntaxKind :: PLUS_EQ } ; [-=] => { $ crate :: SyntaxKind :: MINUS_EQ } ; [*=] => { $ crate :: SyntaxKind :: STAR_EQ } ; [/=] => { $ crate :: SyntaxKind :: SLASH_EQ } ; ["//="] => { $ crate :: SyntaxKind :: SLASH_SLASH_EQ } ; [%=] => { $ crate :: SyntaxKind :: MOD_EQ } ; [&=] => { $ crate :: SyntaxKind :: AND_EQ } ; [|=] => { $ crate :: SyntaxKind :: OR_EQ } ; [^=] => { $ crate :: SyntaxKind :: XOR_EQ } ; [<<=] => { $ crate :: SyntaxKind :: LT_LT_EQ } ; [>>=] => { $ crate :: SyntaxKind :: GT_GT_EQ } ; [whitespace] => { $ crate :: SyntaxKind :: WHITESPACE } ; [ident] => { $ crate :: SyntaxKind :: IDENT } ; [pass] => { $ crate :: SyntaxKind :: PASS_KW } ; [break] => { $ crate :: SyntaxKind :: BREAK_KW } ; [continue] => { $ crate :: SyntaxKind :: CONTINUE_KW } ; ['('] => { $ crate :: SyntaxKind :: L_PAREN } ; ['['] => { $ crate :: SyntaxKind :: L_BRACK } ; ['{'] => { $ crate :: SyntaxKind :: L_BRACE } ; [')'] => { $ crate :: SyntaxKind :: R_PAREN } ; [']'] => { $ crate :: SyntaxKind :: R_BRACK } ; [:] => { $ crate :: SyntaxKind :: COLON } ; [def] => { $ crate :: SyntaxKind :: DEF_KW } ; [,] => { $ crate :: SyntaxKind :: COMMA } ; ['\n'] => { $ crate :: SyntaxKind :: NEWLINE } ; [return] => { $ crate :: SyntaxKind :: RETURN_KW } ; [load] => { $ crate :: SyntaxKind :: LOAD_KW } ; [if] => { $ crate :: SyntaxKind :: IF_KW } ; [else] => { $ crate :: SyntaxKind :: ELSE_KW } ; [for] => { $ crate :: SyntaxKind :: FOR_KW } ; [lambda] => { $ crate :: SyntaxKind :: LAMBDA_KW } ; [not] => { $ crate :: SyntaxKind :: NOT_KW } ; [~] => { $ crate :: SyntaxKind :: TILDE } ; [+] => { $ crate :: SyntaxKind :: PLUS } ; [&] => { $ crate :: SyntaxKind :: AND } ; [|] => { $ crate :: SyntaxKind :: OR } ; [^] => { $ crate :: SyntaxKind :: XOR } ; [and] => { $ crate :: SyntaxKind :: AND_KW } ; [or] => { $ crate :: SyntaxKind :: OR_KW } ; [<<] => { $ crate :: SyntaxKind :: LT_LT } ; [>>] => { $ crate :: SyntaxKind :: GT_GT } ; [.] => { $ crate :: SyntaxKind :: DOT } ; ["//"] => { $ crate :: SyntaxKind :: SLASH_SLASH } ; [%] => { $ crate :: SyntaxKind :: MOD } ; [==] => { $ crate :: SyntaxKind :: EQ_EQ } ; [!=] => { $crate :: SyntaxKind :: BANG_EQ } ; [<] => { $crate :: SyntaxKind :: LT } ; [>] => { $crate :: SyntaxKind :: GT } ; [<=] => { $crate :: SyntaxKind :: LT_EQ } ; [>=] => { $crate :: SyntaxKind :: GT_EQ } ; [in] => { $crate :: SyntaxKind :: IN_KW } ;
}
pub use T;
