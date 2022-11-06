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
    LAMBDA_EXPR,
    DOT_EXPR,
    CALL_EXPR,
    SLICE_EXPR,
    LIST_EXPR,
    DICT_EXPR,
    LIST_COMP,
    DICT_COMP,
    ARGUMENTS,
    ARGUMENT,
    PARAMETERS,
    PARAMETER,
    ENTRIES,
    ENTRY,
    COMP_CLAUSE,
    SUITE,
    LOOP_VARAIBLES,
    FILE,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SyntaxKindSet(u128);

impl SyntaxKindSet {
    pub const fn new() -> SyntaxKindSet {
        SyntaxKindSet(0)
    }

    pub const fn from(kinds: &[SyntaxKind]) -> SyntaxKindSet {
        let mut inner = 0;
        let mut i = 0;
        while i < kinds.len() {
            inner |= 1 << kinds[i] as u16;
            i += 1;
        }
        SyntaxKindSet(inner)
    }

    pub fn contains(&self, kind: SyntaxKind) -> bool {
        self.0 & 1 << kind as usize > 0
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        // fo x in y {}

        Self(kind as u16)
    }
}

#[macro_export]
macro_rules! T {
    [;] => { $ crate :: SyntaxKind :: PLUS } ; [-] => { $ crate :: SyntaxKind :: MINUS } ; [*] => { $ crate :: SyntaxKind :: STAR } ; [/] => { $ crate :: SyntaxKind :: SLASH } ; [/] => { $ crate :: SyntaxKind :: SLASH } ; [whitespace] => { $ crate :: SyntaxKind :: WHITESPACE } ; [ident] => { $ crate :: SyntaxKind :: IDENT } ; [pass] => { $ crate :: SyntaxKind :: PASS_KW } ; [break] => { $ crate :: SyntaxKind :: BREAK_KW } ; [continue] => { $ crate :: SyntaxKind :: CONTINUE_KW } ; ['('] => { $ crate :: SyntaxKind :: L_PAREN } ; [')'] => { $ crate :: SyntaxKind :: R_PAREN } ; [:] => { $ crate :: SyntaxKind :: COLON } ; [def] => { $ crate :: SyntaxKind :: DEF_KW } ; [,] => { $ crate :: SyntaxKind :: COMMA } ;
}
pub use T;
