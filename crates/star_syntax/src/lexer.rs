use super::syntax_kind::SyntaxKind::{self, *};
use std::cmp::Ordering;
use std::str::Chars;

#[derive(Debug)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: usize,
}

#[derive(Debug)]
pub struct LexerReturn(pub Token, pub Option<String>);

pub struct Lexer<'src> {
    chars: Chars<'src>,
    input: &'src str,
    pos: usize,
    depth: usize,
    done: bool,
    indent_emit: isize,
    indent_error: Option<String>,
    indent_levels: Vec<usize>,
    line_start: bool,
    line_pos: Option<usize>,
}

impl<'src> Lexer<'src> {
    pub fn from_str(src: &'src str) -> Self {
        Self {
            chars: src.chars(),
            input: src,
            pos: 0,
            depth: 0,
            indent_emit: 0,
            indent_error: None,
            indent_levels: Vec::with_capacity(10),
            line_start: true,
            line_pos: None,
            done: false,
        }
    }
}

impl<'src> Lexer<'src> {
    fn peek(&mut self) -> Option<char> {
        self.chars.clone().next()
    }

    fn peek2(&mut self) -> Option<char> {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.chars.next();
        if let Some(ch) = ch {
            self.pos += ch.len_utf8();
        }
        ch
    }

    fn parse_single_quoted_string_or_bytes(&mut self, quote: char) -> Option<String> {
        loop {
            match self.peek() {
                Some(ch) if ch == quote => {
                    self.bump();
                    return None;
                }
                Some(_) => {
                    self.bump();
                }
                None => return Some("Unterminated string literal".to_string()),
            }
        }
    }

    fn parse_string_or_bytes(&mut self, quote: char, raw: bool) -> Option<String> {
        let peek_first = self.peek();
        let peek_second = self.peek2();

        // Parse a regular string.
        if raw || peek_first != Some(quote) || peek_second != Some(quote) {
            return self.parse_single_quoted_string_or_bytes(quote);
        }

        self.bump();
        self.bump();

        // Parse triple-quoted string.
        loop {
            match self.peek() {
                Some(ch) if ch == quote => {
                    self.bump();
                    let first = self.bump();
                    let second = self.bump();
                    if first == Some(quote) && second == Some(quote) {
                        break None;
                    }
                    if first.is_none() || second.is_none() {
                        break Some("Unterminated string literal".to_string());
                    }
                }
                Some(_) => {
                    self.bump();
                }
                None => {
                    break Some("Unterminated string literal".to_string());
                }
            }
        }
    }

    fn parse_identifier_or_keyword(&mut self, start: usize) -> SyntaxKind {
        while matches!(self.peek(), Some(ch) if ch.is_alphabetic() || ch.is_ascii_digit()) {
            self.bump();
        }
        match &self.input[start..self.pos] {
            "and" => AND_KW,
            "break" => BREAK_KW,
            "continue" => CONTINUE_KW,
            "def" => DEF_KW,
            "elif" => ELIF_KW,
            "else" => ELSE_KW,
            "for" => FOR_KW,
            "if" => IF_KW,
            "in" => IN_KW,
            "lambda" => LAMBDA_KW,
            "load" => LOAD_KW,
            "not" => NOT_KW,
            "or" => OR_KW,
            "pass" => PASS_KW,
            "return" => RETURN_KW,
            "as" => AS_KW,
            "import" => IMPORT_KW,
            "assert" => ASSERT_KW,
            "is" => IS_KW,
            "class" => CLASS_KW,
            "nonlocal" => NONLOCAL_KW,
            "del" => DEL_KW,
            "raise" => RAISE_KW,
            "except" => EXECPT_KW,
            "try" => TRY_KW,
            "finally" => FINALLY_KW,
            "while" => WHILE_KW,
            "from" => FROM_KW,
            "with" => WITH_KW,
            "global" => GLOBAL_KW,
            "yield" => YIELD_KW,
            _ => IDENT,
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = LexerReturn;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! peek_or {
            ($want:expr, $kind:expr, $alt:expr) => {
                match self.peek() {
                    Some($want) => {
                        self.bump();
                        $kind
                    }
                    _ => $alt,
                }
            };
        }

        // Check if we're not currently in a line continuation from '\'. If we aren't,
        // then calculate indentation.
        let mut indent_cols = 0;
        let was_line_start = self.line_start;

        if self.line_start {
            self.line_start = false;
            self.line_pos = Some(self.pos);
            loop {
                match self.peek() {
                    Some(' ') => {
                        indent_cols += 1;
                    }
                    Some('\t') => {
                        indent_cols += 8;
                    }
                    _ => break,
                }
                self.bump();
            }

            if !matches!(self.peek(), Some('\n' | '#') | None) {
                // Compare the current indentation level to the previous level. There are four cases:
                //   - Equal to previous: Don't need to emit any INDENT/OUTDENT tokens.
                //   - Greater than previous: Emit an INDENT token, push this indentation level to the stack.
                //   - Less than previous: Pop indents off the stack until the top element is less/equal than the previous.
                //     For each indent popped, we will emit an OUTDENT token in the next step. Once the loop finishes,
                //     our indentation level must be equal to the top element in the stack, or if the stack is empty, it must
                //     be 0. Otherwise, we have invalid indentation. We recover by appending an indentation diagnostic to the next
                //     token.
                let indent_level = self.indent_levels.last().cloned().unwrap_or(0);
                match indent_cols.cmp(&indent_level) {
                    Ordering::Less => {
                        self.indent_levels.pop();
                        self.indent_emit -= 1;
                        loop {
                            let indent_level = self.indent_levels.last().cloned().unwrap_or(0);
                            match indent_cols.cmp(&indent_level) {
                                Ordering::Less => {
                                    self.indent_levels.pop();
                                    self.indent_emit -= 1;
                                }
                                Ordering::Equal => break,
                                Ordering::Greater => {
                                    self.indent_error = Some(
                                        "Dedent amount does not match previous indentation"
                                            .to_string(),
                                    );
                                    break;
                                }
                            }
                        }
                    }
                    Ordering::Greater => {
                        self.indent_emit += 1;
                        self.indent_levels.push(indent_cols);
                    }
                    Ordering::Equal => {}
                }
            }
        }

        // Emit WHITESPACE consumed while calculating the indentation level.
        if indent_cols > 0 {
            return Some(LexerReturn(
                Token {
                    kind: WHITESPACE,
                    len: indent_cols,
                },
                None,
            ));
        }

        // Emit stored INDENT/OUTDENT tokens.
        match self.indent_emit.cmp(&0) {
            Ordering::Less => {
                self.indent_emit += 1;
                return Some(LexerReturn(
                    Token {
                        kind: OUTDENT,
                        len: 0,
                    },
                    self.indent_error.take(),
                ));
            }
            Ordering::Greater => {
                self.indent_emit -= 1;
                return Some(LexerReturn(
                    Token {
                        kind: INDENT,
                        len: 0,
                    },
                    None,
                ));
            }
            _ => {}
        }

        let mut diagnostic = None;
        let token_start = self.pos;

        let kind = match self.bump() {
            Some(ch) => match ch {
                ch if ch.is_alphabetic() => {
                    // Check for bytes, raw string, or raw bytes literals.
                    let peek_first = self.peek();
                    let peek_second = self.peek2();
                    let mut kind = STRING;
                    match (ch, peek_first, peek_second) {
                        ('r' | 'b', Some(ch @ ('\'' | '"')), _) => {
                            self.bump();
                            diagnostic = self.parse_string_or_bytes(ch, ch == 'r');
                        }
                        ('r', Some('b'), Some(ch @ ('\'' | '"')))
                        | ('b', Some('r'), Some(ch @ ('\'' | '"'))) => {
                            self.bump();
                            self.bump();
                            // raw bytes (br)
                            diagnostic = self.parse_string_or_bytes(ch, true);
                        }
                        _ => {
                            kind = self.parse_identifier_or_keyword(token_start);
                        }
                    }
                    kind
                }
                '0'..='9' => {
                    while matches!(self.peek(), Some('0'..='9')) {
                        self.bump();
                    }
                    INT
                }
                '\'' | '"' => {
                    diagnostic = self.parse_string_or_bytes(ch, false);
                    STRING
                }
                ' ' | '\t' | '\r' => {
                    while matches!(self.peek(), Some(' ' | '\t' | '\r')) {
                        self.bump();
                    }
                    WHITESPACE
                }
                '#' => {
                    while self.peek() != Some('\n') {
                        self.bump();
                    }
                    COMMENT
                }
                '+' => peek_or!('=', PLUS_EQ, PLUS),
                '-' => peek_or!('=', MINUS_EQ, MINUS),
                '*' => match self.peek() {
                    Some('*') => {
                        self.bump();
                        STAR_STAR
                    }
                    Some('=') => {
                        self.bump();
                        STAR_EQ
                    }
                    _ => STAR,
                },
                '/' => match self.peek() {
                    Some('/') => {
                        self.bump();
                        if matches!(self.peek(), Some('=')) {
                            self.bump();
                            SLASH_SLASH_EQ
                        } else {
                            SLASH_SLASH
                        }
                    }
                    Some('=') => {
                        self.bump();
                        SLASH_EQ
                    }
                    _ => SLASH,
                },
                '%' => peek_or!('=', MOD_EQ, MOD),
                '~' => TILDE,
                '&' => peek_or!('&', AND_EQ, AND),
                '|' => peek_or!('|', OR_EQ, OR),
                '^' => peek_or!('^', XOR_EQ, XOR),
                '<' => todo!(),
                '>' => todo!(),
                '.' => DOT,
                ',' => COMMA,
                '=' => peek_or!('=', EQ_EQ, EQ),
                ';' => SEMICOLON,
                ':' => COLON,
                '(' | '[' | '{' => {
                    self.depth += 1;
                    match ch {
                        '(' => L_PAREN,
                        '[' => L_BRACK,
                        '{' => L_BRACE,
                        _ => unreachable!(),
                    }
                }
                ')' | ']' | '}' => {
                    self.depth = self.depth.saturating_sub(1);
                    match ch {
                        ')' => R_PAREN,
                        ']' => R_BRACK,
                        '}' => R_BRACE,
                        _ => unreachable!(),
                    }
                }
                '\n' => {
                    if self.depth > 0 {
                        // TODO: This causes fragmented WHITESPACE tokens, should be coalesced.
                        WHITESPACE
                    } else {
                        self.line_start = true;
                        NEWLINE
                    }
                }
                _ => {
                    diagnostic = Some("Unexpected character".to_string());
                    ERROR
                }
            },
            None => {
                let was_done = self.done;
                self.done = true;

                // Emit this NEWLINE at most once.
                if !was_line_start && !was_done {
                    self.line_start = true;
                    NEWLINE
                } else if self.indent_levels.pop().is_some() {
                    OUTDENT
                } else {
                    return None;
                }
            }
        };

        Some(LexerReturn(
            Token {
                kind,
                len: self.pos - token_start,
            },
            diagnostic,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, LexerReturn};
    use expect_test::{expect, Expect};

    fn check_lexing(input: &str, expect: Expect) {
        let mut pos = 0;
        let actual: String = Lexer::from_str(input)
            .map(|LexerReturn(token, diagnostic)| {
                let start = pos;
                pos += token.len;
                format!(
                    "{:?}@{}..{} {:?} {:?}\n",
                    token.kind,
                    start,
                    pos,
                    &input[start..pos],
                    diagnostic,
                )
            })
            .collect();
        expect.assert_eq(&actual);
    }

    #[test]
    fn test_ident() {
        check_lexing(
            r#"foo
    r'bar'
        baz
  qux
baz

for foo in bar
"#,
            expect![[r#"
                IDENT@0..3 "foo" None
                NEWLINE@3..4 "\n" None
                WHITESPACE@4..8 "    " None
                INDENT@8..8 "" None
                STRING@8..14 "r'bar'" None
                NEWLINE@14..15 "\n" None
                WHITESPACE@15..23 "        " None
                INDENT@23..23 "" None
                IDENT@23..26 "baz" None
                NEWLINE@26..27 "\n" None
                WHITESPACE@27..29 "  " None
                OUTDENT@29..29 "" Some("Dedent amount does not match previous indentation")
                OUTDENT@29..29 "" None
                IDENT@29..32 "qux" None
                NEWLINE@32..33 "\n" None
                IDENT@33..36 "baz" None
                NEWLINE@36..37 "\n" None
                NEWLINE@37..38 "\n" None
                FOR_KW@38..41 "for" None
                WHITESPACE@41..42 " " None
                IDENT@42..45 "foo" None
                WHITESPACE@45..46 " " None
                IN_KW@46..48 "in" None
                WHITESPACE@48..49 " " None
                IDENT@49..52 "bar" None
                NEWLINE@52..53 "\n" None
            "#]],
        );
    }

    #[test]
    fn test_strings() {
        check_lexing(
            r#"
"foo"
'foo'
'''foo"'''
"""foo'"""
r'hello'
r"hello"
b"hello"
b'hello'
b'''hello"'''
b"""hello'"""
rb'hello'
br"hello""#,
            expect![[r#"
                NEWLINE@0..1 "\n" None
                STRING@1..6 "\"foo\"" None
                NEWLINE@6..7 "\n" None
                STRING@7..12 "'foo'" None
                NEWLINE@12..13 "\n" None
                STRING@13..23 "'''foo\"'''" None
                NEWLINE@23..24 "\n" None
                STRING@24..34 "\"\"\"foo'\"\"\"" None
                NEWLINE@34..35 "\n" None
                STRING@35..43 "r'hello'" None
                NEWLINE@43..44 "\n" None
                STRING@44..52 "r\"hello\"" None
                NEWLINE@52..53 "\n" None
                STRING@53..61 "b\"hello\"" None
                NEWLINE@61..62 "\n" None
                STRING@62..70 "b'hello'" None
                NEWLINE@70..71 "\n" None
                STRING@71..84 "b'''hello\"'''" None
                NEWLINE@84..85 "\n" None
                STRING@85..98 "b\"\"\"hello'\"\"\"" None
                NEWLINE@98..99 "\n" None
                STRING@99..108 "rb'hello'" None
                NEWLINE@108..109 "\n" None
                STRING@109..118 "br\"hello\"" None
                NEWLINE@118..118 "" None
            "#]],
        )
    }

    #[test]
    fn smoke_test() {
        check_lexing(
            r#"
   # module a.sky
def f(x, list=[]):
  list.append(x)
  return list

f(4, [1,2,3])           # [1, 2, 3, 4]
f(1)                    # [1]
f(2)                    # [1, 2], not [2]!

# module b.sky
load("a.sky", "f")
f(3)  
"#,
            expect![[r##"
                NEWLINE@0..1 "\n" None
                WHITESPACE@1..4 "   " None
                COMMENT@4..18 "# module a.sky" None
                NEWLINE@18..19 "\n" None
                DEF_KW@19..22 "def" None
                WHITESPACE@22..23 " " None
                IDENT@23..24 "f" None
                L_PAREN@24..25 "(" None
                IDENT@25..26 "x" None
                COMMA@26..27 "," None
                WHITESPACE@27..28 " " None
                IDENT@28..32 "list" None
                EQ@32..33 "=" None
                L_BRACK@33..34 "[" None
                R_BRACK@34..35 "]" None
                R_PAREN@35..36 ")" None
                COLON@36..37 ":" None
                NEWLINE@37..38 "\n" None
                WHITESPACE@38..40 "  " None
                INDENT@40..40 "" None
                IDENT@40..44 "list" None
                DOT@44..45 "." None
                IDENT@45..51 "append" None
                L_PAREN@51..52 "(" None
                IDENT@52..53 "x" None
                R_PAREN@53..54 ")" None
                NEWLINE@54..55 "\n" None
                WHITESPACE@55..57 "  " None
                RETURN_KW@57..63 "return" None
                WHITESPACE@63..64 " " None
                IDENT@64..68 "list" None
                NEWLINE@68..69 "\n" None
                NEWLINE@69..70 "\n" None
                OUTDENT@70..70 "" None
                IDENT@70..71 "f" None
                L_PAREN@71..72 "(" None
                INT@72..73 "4" None
                COMMA@73..74 "," None
                WHITESPACE@74..75 " " None
                L_BRACK@75..76 "[" None
                INT@76..77 "1" None
                COMMA@77..78 "," None
                INT@78..79 "2" None
                COMMA@79..80 "," None
                INT@80..81 "3" None
                R_BRACK@81..82 "]" None
                R_PAREN@82..83 ")" None
                WHITESPACE@83..94 "           " None
                COMMENT@94..108 "# [1, 2, 3, 4]" None
                NEWLINE@108..109 "\n" None
                IDENT@109..110 "f" None
                L_PAREN@110..111 "(" None
                INT@111..112 "1" None
                R_PAREN@112..113 ")" None
                WHITESPACE@113..133 "                    " None
                COMMENT@133..138 "# [1]" None
                NEWLINE@138..139 "\n" None
                IDENT@139..140 "f" None
                L_PAREN@140..141 "(" None
                INT@141..142 "2" None
                R_PAREN@142..143 ")" None
                WHITESPACE@143..163 "                    " None
                COMMENT@163..181 "# [1, 2], not [2]!" None
                NEWLINE@181..182 "\n" None
                NEWLINE@182..183 "\n" None
                COMMENT@183..197 "# module b.sky" None
                NEWLINE@197..198 "\n" None
                LOAD_KW@198..202 "load" None
                L_PAREN@202..203 "(" None
                STRING@203..210 "\"a.sky\"" None
                COMMA@210..211 "," None
                WHITESPACE@211..212 " " None
                STRING@212..215 "\"f\"" None
                R_PAREN@215..216 ")" None
                NEWLINE@216..217 "\n" None
                IDENT@217..218 "f" None
                L_PAREN@218..219 "(" None
                INT@219..220 "3" None
                R_PAREN@220..221 ")" None
                WHITESPACE@221..223 "  " None
                NEWLINE@223..224 "\n" None
            "##]],
        )
    }

    #[test]
    fn newline_whitespace() {
        check_lexing(
            r#"
def foo():
    def bar():
        def baz():
        x = 1

x = 1"#,
            expect![[r#"
                NEWLINE@0..1 "\n" None
                DEF_KW@1..4 "def" None
                WHITESPACE@4..5 " " None
                IDENT@5..8 "foo" None
                L_PAREN@8..9 "(" None
                R_PAREN@9..10 ")" None
                COLON@10..11 ":" None
                NEWLINE@11..12 "\n" None
                WHITESPACE@12..16 "    " None
                INDENT@16..16 "" None
                DEF_KW@16..19 "def" None
                WHITESPACE@19..20 " " None
                IDENT@20..23 "bar" None
                L_PAREN@23..24 "(" None
                R_PAREN@24..25 ")" None
                COLON@25..26 ":" None
                NEWLINE@26..27 "\n" None
                WHITESPACE@27..35 "        " None
                INDENT@35..35 "" None
                DEF_KW@35..38 "def" None
                WHITESPACE@38..39 " " None
                IDENT@39..42 "baz" None
                L_PAREN@42..43 "(" None
                R_PAREN@43..44 ")" None
                COLON@44..45 ":" None
                NEWLINE@45..46 "\n" None
                WHITESPACE@46..54 "        " None
                IDENT@54..55 "x" None
                WHITESPACE@55..56 " " None
                EQ@56..57 "=" None
                WHITESPACE@57..58 " " None
                INT@58..59 "1" None
                NEWLINE@59..60 "\n" None
                NEWLINE@60..61 "\n" None
                OUTDENT@61..61 "" None
                OUTDENT@61..61 "" None
                IDENT@61..62 "x" None
                WHITESPACE@62..63 " " None
                EQ@63..64 "=" None
                WHITESPACE@64..65 " " None
                INT@65..66 "1" None
                NEWLINE@66..66 "" None
            "#]],
        )
    }
}
