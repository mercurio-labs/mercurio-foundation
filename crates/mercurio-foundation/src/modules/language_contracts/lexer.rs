use crate::language_contracts::ast::{CommentKind, SourceSpan};
use crate::language_contracts::diagnostics::Diagnostic;

/// A `//` or non-doc `/* */` comment skipped before a token. Doc-candidate
/// blocks (the `comment … /* */` usage body) and `doc` bodies are language
/// content, not trivia, and are never collected here. The text is the raw
/// interior — after `//`, or between `/*` and `*/` — so re-rendering
/// `//{text}` or `/*{text}*/` reproduces the original bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentTrivia {
    pub text: String,
    pub kind: CommentKind,
    pub span: SourceSpan,
    /// True when nothing but whitespace precedes the comment on its line;
    /// false for a trailing comment after code on the same line.
    pub own_line: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: SourceSpan,
    /// Comment trivia skipped between the previous token and this one.
    pub leading_trivia: Vec<CommentTrivia>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Package,
    Import,
    Part,
    Def,
    Specializes,
    Redefines,
    Doc(String),
    BlockDoc(String),
    Identifier(String),
    Number(String),
    String(String),
    Star,
    DoubleStar,
    Caret,
    LBrace,
    RBrace,
    LAngle,
    RAngle,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Colon,
    ScopeSep,
    Dot,
    Tilde,
    Equals,
    DoubleEquals,
    BangEquals,
    Plus,
    Minus,
    LessEqual,
    GreaterEqual,
    Slash,
    Percent,
    Bang,
    Ampersand,
    Pipe,
    Dollar,
    At,
    Hash,
    Question,
    Semicolon,
    Comma,
    Eof,
}

pub fn lex(input: &str) -> Result<Vec<Token>, Diagnostic> {
    let mut lexer = Lexer::new(input);
    lexer.lex_all()
}

/// Optional language-owned terminal prefix recognizer. Its result is a UTF-8
/// byte length; token construction and source spelling stay in this adapter.
pub type TerminalMatcher = fn(&str, &str) -> Option<usize>;

pub fn lex_with_terminal_matcher(
    input: &str,
    matcher: TerminalMatcher,
) -> Result<Vec<Token>, Diagnostic> {
    let mut lexer = Lexer::new(input);
    lexer.terminal_matcher = Some(matcher);
    lexer.lex_all()
}

/// Language-owned hidden-token match. Byte offsets retain original source spans.
pub struct HiddenTokenMatch {
    pub length: usize,
    pub comment: Option<(CommentKind, std::ops::Range<usize>)>,
}
pub type HiddenTokenMatcher = fn(&str) -> Result<Option<HiddenTokenMatch>, &'static str>;

pub fn lex_with_language_matchers(
    input: &str,
    terminal: TerminalMatcher,
    hidden: HiddenTokenMatcher,
) -> Result<Vec<Token>, Diagnostic> {
    let mut lexer = Lexer::new(input);
    lexer.terminal_matcher = Some(terminal);
    lexer.hidden_matcher = Some(hidden);
    lexer.lex_all()
}

/// Language-owned span of an already identified numeric carrier. The shared
/// scanner validates the byte range and preserves its spelling and source span.
pub type NumberMatcher = fn(&str) -> Result<usize, &'static str>;

pub fn lex_with_numeric_matcher(
    input: &str,
    terminal: TerminalMatcher,
    hidden: HiddenTokenMatcher,
    number: NumberMatcher,
) -> Result<Vec<Token>, Diagnostic> {
    let mut lexer = Lexer::new(input);
    lexer.terminal_matcher = Some(terminal);
    lexer.hidden_matcher = Some(hidden);
    lexer.number_matcher = Some(number);
    lexer.lex_all()
}

struct Lexer<'a> {
    input: &'a str,
    bytes: &'a [u8],
    index: usize,
    line: usize,
    col: usize,
    pending_trivia: Vec<CommentTrivia>,
    last_content_line: usize,
    terminal_matcher: Option<TerminalMatcher>,
    hidden_matcher: Option<HiddenTokenMatcher>,
    number_matcher: Option<NumberMatcher>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            bytes: input.as_bytes(),
            index: 0,
            line: 1,
            col: 1,
            pending_trivia: Vec::new(),
            last_content_line: 0,
            terminal_matcher: None,
            hidden_matcher: None,
            number_matcher: None,
        }
    }

    fn lex_all(&mut self) -> Result<Vec<Token>, Diagnostic> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments()?;
            let start_line = self.line;
            let start_col = self.col;
            let Some(ch) = self.peek_char() else {
                tokens.push(Token {
                    kind: TokenKind::Eof,
                    span: SourceSpan {
                        start_line,
                        start_col,
                        end_line: start_line,
                        end_col: start_col,
                    },
                    leading_trivia: std::mem::take(&mut self.pending_trivia),
                });
                return Ok(tokens);
            };

            let kind = match ch {
                '{' => {
                    self.advance_char();
                    TokenKind::LBrace
                }
                '}' => {
                    self.advance_char();
                    TokenKind::RBrace
                }
                '<' => {
                    self.advance_char();
                    if self.peek_char() == Some('=') {
                        self.advance_char();
                        TokenKind::LessEqual
                    } else {
                        TokenKind::LAngle
                    }
                }
                '>' => {
                    self.advance_char();
                    if self.peek_char() == Some('=') {
                        self.advance_char();
                        TokenKind::GreaterEqual
                    } else {
                        TokenKind::RAngle
                    }
                }
                '[' => {
                    self.advance_char();
                    TokenKind::LBracket
                }
                ']' => {
                    self.advance_char();
                    TokenKind::RBracket
                }
                '(' => {
                    self.advance_char();
                    TokenKind::LParen
                }
                ')' => {
                    self.advance_char();
                    TokenKind::RParen
                }
                ':' => {
                    self.advance_char();
                    if self.peek_char() == Some(':') {
                        self.advance_char();
                        TokenKind::ScopeSep
                    } else if self.peek_char() == Some('>') {
                        self.advance_char();
                        if self.peek_char() == Some('>') {
                            self.advance_char();
                            TokenKind::Redefines
                        } else {
                            TokenKind::Specializes
                        }
                    } else {
                        TokenKind::Colon
                    }
                }
                '*' => {
                    self.advance_char();
                    if self.peek_char() == Some('*') {
                        self.advance_char();
                        TokenKind::DoubleStar
                    } else {
                        TokenKind::Star
                    }
                }
                ';' => {
                    self.advance_char();
                    TokenKind::Semicolon
                }
                ',' => {
                    self.advance_char();
                    TokenKind::Comma
                }
                '.' => {
                    if (self.index == 0 || self.bytes[self.index - 1] != b'.')
                        && self
                            .peek_next_char()
                            .is_some_and(|next| next.is_ascii_digit())
                    {
                        TokenKind::Number(self.lex_fractional_number()?)
                    } else {
                        self.advance_char();
                        TokenKind::Dot
                    }
                }
                '~' => {
                    self.advance_char();
                    TokenKind::Tilde
                }
                '=' => {
                    self.advance_char();
                    if self.peek_char() == Some('=') {
                        self.advance_char();
                        TokenKind::DoubleEquals
                    } else {
                        TokenKind::Equals
                    }
                }
                '+' => {
                    self.advance_char();
                    TokenKind::Plus
                }
                '^' => {
                    self.advance_char();
                    TokenKind::Caret
                }
                '-' => {
                    self.advance_char();
                    TokenKind::Minus
                }
                '/' if self.peek_next_char() == Some('*') => {
                    TokenKind::BlockDoc(self.consume_doc_block()?)
                }
                '%' => {
                    self.advance_char();
                    TokenKind::Percent
                }
                '/' => {
                    self.advance_char();
                    TokenKind::Slash
                }
                '!' => {
                    self.advance_char();
                    if self.peek_char() == Some('=') {
                        self.advance_char();
                        TokenKind::BangEquals
                    } else {
                        TokenKind::Bang
                    }
                }
                '&' => {
                    self.advance_char();
                    TokenKind::Ampersand
                }
                '|' => {
                    self.advance_char();
                    TokenKind::Pipe
                }
                '$' => {
                    self.advance_char();
                    TokenKind::Dollar
                }
                '@' => {
                    self.advance_char();
                    TokenKind::At
                }
                '#' => {
                    self.advance_char();
                    TokenKind::Hash
                }
                '?' => {
                    self.advance_char();
                    TokenKind::Question
                }
                '\'' => TokenKind::Identifier(self.lex_quoted_identifier()?),
                '"' => TokenKind::String(self.lex_string_literal()?),
                _ if ch.is_ascii_digit() => TokenKind::Number(self.lex_number()?),
                _ if is_ident_start(ch) => self.lex_identifier_or_keyword()?,
                _ => {
                    return Err(Diagnostic::new(
                        format!("unexpected character `{ch}`"),
                        Some(SourceSpan {
                            start_line,
                            start_col,
                            end_line: start_line,
                            end_col: start_col,
                        }),
                    ));
                }
            };

            tokens.push(Token {
                kind,
                span: SourceSpan {
                    start_line,
                    start_col,
                    end_line: self.line,
                    end_col: self.col.saturating_sub(1),
                },
                leading_trivia: std::mem::take(&mut self.pending_trivia),
            });
            self.last_content_line = self.line;
        }
    }

    fn lex_generated_number(&mut self, matcher: NumberMatcher) -> Result<String, Diagnostic> {
        let remaining = &self.input[self.index..];
        let length = matcher(remaining)
            .and_then(|length| {
                if length > 0 && remaining.get(..length).is_some() {
                    Ok(length)
                } else {
                    Err("invalid generated number boundaries")
                }
            })
            .map_err(|message| {
                Diagnostic::new(
                    message,
                    Some(SourceSpan {
                        start_line: self.line,
                        start_col: self.col,
                        end_line: self.line,
                        end_col: self.col,
                    }),
                )
            })?;
        let spelling = remaining[..length].to_string();
        let end = self.index + length;
        while self.index < end {
            self.advance_char();
        }
        Ok(spelling)
    }

    fn lex_number(&mut self) -> Result<String, Diagnostic> {
        if let Some(matcher) = self.number_matcher {
            return self.lex_generated_number(matcher);
        }
        let start = self.index;
        let mut saw_decimal = false;
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_digit() {
                self.advance_char();
            } else if ch == '.'
                && !saw_decimal
                && self
                    .peek_next_char()
                    .is_some_and(|next| next.is_ascii_digit())
            {
                saw_decimal = true;
                self.advance_char();
            } else {
                break;
            }
        }
        self.lex_exponent()?;
        Ok(self.input[start..self.index].to_string())
    }

    fn lex_fractional_number(&mut self) -> Result<String, Diagnostic> {
        if let Some(matcher) = self.number_matcher {
            return self.lex_generated_number(matcher);
        }
        let start = self.index;
        self.advance_char();
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_digit() {
                self.advance_char();
            } else {
                break;
            }
        }
        self.lex_exponent()?;
        Ok(self.input[start..self.index].to_string())
    }

    fn lex_exponent(&mut self) -> Result<(), Diagnostic> {
        if !matches!(self.peek_char(), Some('e' | 'E')) {
            return Ok(());
        }
        let start_line = self.line;
        let start_col = self.col;
        self.advance_char();
        if matches!(self.peek_char(), Some('+' | '-')) {
            self.advance_char();
        }
        if !self.peek_char().is_some_and(|ch| ch.is_ascii_digit()) {
            return Err(Diagnostic::new(
                "expected digits after numeric exponent",
                Some(SourceSpan {
                    start_line,
                    start_col,
                    end_line: self.line,
                    end_col: self.col,
                }),
            ));
        }
        while self.peek_char().is_some_and(|ch| ch.is_ascii_digit()) {
            self.advance_char();
        }
        Ok(())
    }

    fn lex_generated_delimited(&mut self, rule: &str) -> Option<Result<String, Diagnostic>> {
        self.lex_generated_span(rule, 1, 1)
    }

    fn lex_generated_span(
        &mut self,
        rule: &str,
        prefix: usize,
        suffix: usize,
    ) -> Option<Result<String, Diagnostic>> {
        let matcher = self.terminal_matcher?;
        let remaining = &self.input[self.index..];
        let length = matcher(rule, remaining).filter(|length| {
            *length > 0 && *length >= prefix + suffix && remaining.get(..*length).is_some()
        });
        let Some(length) = length else {
            return Some(Err(Diagnostic::new(
                format!("invalid or unterminated {rule} terminal"),
                Some(SourceSpan {
                    start_line: self.line,
                    start_col: self.col,
                    end_line: self.line,
                    end_col: self.col,
                }),
            )));
        };
        let Some(interior) = remaining.get(prefix..length - suffix) else {
            return Some(Err(Diagnostic::new(
                "invalid generated delimited-token boundaries",
                None,
            )));
        };
        let value = interior.to_owned();
        let end = self.index + length;
        while self.index < end {
            self.advance_char();
        }
        Some(Ok(value))
    }

    fn lex_quoted_identifier(&mut self) -> Result<String, Diagnostic> {
        if let Some(result) = self.lex_generated_delimited("UNRESTRICTED_NAME") {
            return result;
        }
        let start_line = self.line;
        let start_col = self.col;
        self.advance_char();
        let start = self.index;

        while let Some(ch) = self.peek_char() {
            if ch == '\'' {
                let ident = self.input[start..self.index].to_string();
                self.advance_char();
                return Ok(ident);
            }
            self.advance_char();
        }

        Err(Diagnostic::new(
            "unterminated quoted identifier",
            Some(SourceSpan {
                start_line,
                start_col,
                end_line: self.line,
                end_col: self.col,
            }),
        ))
    }

    fn lex_string_literal(&mut self) -> Result<String, Diagnostic> {
        if let Some(result) = self.lex_generated_delimited("STRING_VALUE") {
            return result;
        }
        let start_line = self.line;
        let start_col = self.col;
        self.advance_char();
        let start = self.index;

        while let Some(ch) = self.peek_char() {
            // Retain source spelling, but an escaped quote cannot end a string.
            if ch == '\\' {
                self.advance_char();
                if self.peek_char().is_some() {
                    self.advance_char();
                }
                continue;
            }
            if ch == '"' {
                let value = self.input[start..self.index].to_string();
                self.advance_char();
                return Ok(value);
            }
            self.advance_char();
        }

        Err(Diagnostic::new(
            "unterminated string literal",
            Some(SourceSpan {
                start_line,
                start_col,
                end_line: self.line,
                end_col: self.col,
            }),
        ))
    }

    fn skip_whitespace_and_comments(&mut self) -> Result<(), Diagnostic> {
        if let Some(matcher) = self.hidden_matcher {
            loop {
                let start_line = self.line;
                let start_col = self.col;
                let diagnostic = |message| {
                    Diagnostic::new(
                        message,
                        Some(SourceSpan {
                            start_line,
                            start_col,
                            end_line: start_line,
                            end_col: start_col,
                        }),
                    )
                };
                let Some(hidden) = matcher(&self.input[self.index..]).map_err(diagnostic)? else {
                    return Ok(());
                };
                if hidden.length == 0 || self.input[self.index..].get(..hidden.length).is_none() {
                    return Err(diagnostic("invalid hidden-token length"));
                }
                let end = self.index + hidden.length;
                let comment = if let Some((kind, range)) = hidden.comment {
                    let text = self.input[self.index..end]
                        .get(range)
                        .ok_or_else(|| diagnostic("invalid hidden-token comment range"))?;
                    Some((kind, text.to_owned()))
                } else {
                    None
                };
                let own_line = start_line > self.last_content_line;
                while self.index < end {
                    self.advance_char();
                }
                if let Some((kind, text)) = comment {
                    self.push_comment_trivia(text, kind, start_line, start_col, own_line);
                }
            }
        }
        loop {
            match self.peek_char() {
                Some(ch) if ch.is_whitespace() => {
                    self.advance_char();
                }
                Some('/')
                    if self.peek_next_char() == Some('/')
                        && self.bytes.get(self.index + 2) == Some(&b'*') =>
                {
                    let start_line = self.line;
                    let start_col = self.col;
                    let own_line = start_line > self.last_content_line;
                    let text = self.consume_line_prefixed_block_comment()?;
                    self.push_comment_trivia(
                        text,
                        CommentKind::Block,
                        start_line,
                        start_col,
                        own_line,
                    );
                }
                Some('/') if self.peek_next_char() == Some('/') => {
                    let start_line = self.line;
                    let start_col = self.col;
                    let own_line = start_line > self.last_content_line;
                    self.advance_char();
                    self.advance_char();
                    let text_start = self.index;
                    let mut text_end = self.index;
                    while let Some(ch) = self.peek_char() {
                        if ch == '\n' {
                            self.advance_char();
                            break;
                        }
                        self.advance_char();
                        text_end = self.index;
                    }
                    let text = self.input[text_start..text_end]
                        .trim_end_matches('\r')
                        .to_string();
                    self.push_comment_trivia(
                        text,
                        CommentKind::Line,
                        start_line,
                        start_col,
                        own_line,
                    );
                }
                // KerML/SysML REGULAR_COMMENT is a semantic Comment body.
                // Only // and //* ... */ are lexical trivia.
                Some('/') if self.peek_next_char() == Some('*') => return Ok(()),
                _ => return Ok(()),
            }
        }
    }

    fn lex_identifier_or_keyword(&mut self) -> Result<TokenKind, Diagnostic> {
        let ident = if let Some(result) = self.lex_generated_span("ID", 0, 0) {
            result?
        } else {
            let start = self.index;
            while self.peek_char().is_some_and(is_ident_continue) {
                self.advance_char();
            }
            self.input[start..self.index].to_string()
        };

        match ident.as_str() {
            "package" => Ok(TokenKind::Package),
            "import" => Ok(TokenKind::Import),
            "part" => Ok(TokenKind::Part),
            "def" => Ok(TokenKind::Def),
            "specializes" => Ok(TokenKind::Specializes),
            "doc" => {
                // Keep every documentation declaration as syntax, including anonymous
                // bodies, so parsing retains its source identity and raw editable text.
                Ok(TokenKind::Identifier("doc".into()))
            }
            _ => Ok(TokenKind::Identifier(ident.to_string())),
        }
    }

    fn consume_doc_block(&mut self) -> Result<String, Diagnostic> {
        if let Some(result) = self.lex_generated_span("REGULAR_COMMENT", 2, 2) {
            return result;
        }
        if self.peek_char() != Some('/') || self.peek_next_char() != Some('*') {
            return Err(Diagnostic::new(
                "expected block comment after `doc`",
                Some(SourceSpan {
                    start_line: self.line,
                    start_col: self.col,
                    end_line: self.line,
                    end_col: self.col,
                }),
            ));
        }

        self.advance_char();
        self.advance_char();
        let start = self.index;

        while let Some(ch) = self.peek_char() {
            if ch == '*' && self.peek_next_char() == Some('/') {
                let raw = &self.input[start..self.index];
                let body = raw.to_string();
                self.advance_char();
                self.advance_char();
                return Ok(body);
            }
            self.advance_char();
        }

        Err(Diagnostic::new("unterminated doc block", None))
    }

    fn consume_line_prefixed_block_comment(&mut self) -> Result<String, Diagnostic> {
        self.advance_char();
        self.advance_char();
        self.advance_char();
        self.consume_block_comment_interior()
    }

    fn consume_block_comment_interior(&mut self) -> Result<String, Diagnostic> {
        let start = self.index;
        while let Some(ch) = self.peek_char() {
            if ch == '*' && self.peek_next_char() == Some('/') {
                let text = self.input[start..self.index].to_string();
                self.advance_char();
                self.advance_char();
                return Ok(text);
            }
            self.advance_char();
        }

        Err(Diagnostic::new("unterminated block comment", None))
    }

    fn push_comment_trivia(
        &mut self,
        text: String,
        kind: CommentKind,
        start_line: usize,
        start_col: usize,
        own_line: bool,
    ) {
        self.pending_trivia.push(CommentTrivia {
            text,
            kind,
            span: SourceSpan {
                start_line,
                start_col,
                end_line: self.line,
                end_col: self.col.saturating_sub(1),
            },
            own_line,
        });
    }

    fn peek_char(&self) -> Option<char> {
        self.bytes.get(self.index).map(|byte| *byte as char)
    }

    fn peek_next_char(&self) -> Option<char> {
        self.bytes.get(self.index + 1).map(|byte| *byte as char)
    }

    fn advance_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.index += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::{CommentKind, TokenKind, lex};

    #[test]
    fn lexes_minimal_model_subset() {
        let tokens = lex(
            "package Demo { doc /* hi */ part def Vehicle specializes Model::Systems::PartDefinition { part engine: Engine; } }",
        )
        .unwrap();

        assert!(matches!(tokens[0].kind, TokenKind::Package));
        assert!(
            tokens
                .iter()
                .any(|token| matches!(&token.kind, TokenKind::BlockDoc(value) if value == " hi "))
        );
        assert!(tokens.iter().any(
            |token| matches!(&token.kind, TokenKind::Identifier(value) if value == "Vehicle")
        ));
    }

    #[test]
    fn lexes_quoted_identifiers_and_wildcards() {
        let tokens =
            lex("package 'Port Example' { import ScalarValues::*; import Pkg::*::**; }").unwrap();

        assert!(tokens.iter().any(
            |token| matches!(&token.kind, TokenKind::Identifier(value) if value == "Port Example")
        ));
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Star))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::DoubleStar))
        );
    }

    #[test]
    fn lexes_specialization_shorthand() {
        let tokens = lex("part def Engine :> Vehicle;").unwrap();

        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Specializes))
        );
    }

    #[test]
    fn lexes_loose_surface_syntax_used_in_pilot_examples() {
        let tokens = lex("part <'1'> b[0..2]: ~C = X::y(0) { part x; }").unwrap();

        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::LAngle))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::RAngle))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::LBracket))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::RBracket))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::LParen))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::RParen))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Dot))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Tilde))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Equals))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(&token.kind, TokenKind::Number(value) if value == "0"))
        );
    }

    #[test]
    fn lexes_double_gt_relation_shorthand() {
        let tokens = lex("part x :>> y;").unwrap();

        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Redefines))
        );
    }

    #[test]
    fn lexes_arithmetic_and_annotation_surface_syntax() {
        let tokens =
            lex("calc def Power { return : Value = a + b - c / d & e; @tag #note }").unwrap();

        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Plus))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Minus))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Slash))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Ampersand))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::At))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Hash))
        );
    }

    #[test]
    fn lexes_expression_operator_tokens() {
        let tokens =
            lex("attribute x = (1 + 2) * 3 >= 4 and not false == !false != true;").unwrap();

        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Star))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::GreaterEqual))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::DoubleEquals))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::BangEquals))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Bang))
        );
    }

    #[test]
    fn lexes_named_doc_and_string_literals() {
        let tokens = lex("doc Document1 /* hi */ attribute code = \"uncl\"; opaque ?;").unwrap();

        assert!(matches!(&tokens[0].kind, TokenKind::Identifier(value) if value == "doc"));
        assert!(matches!(&tokens[1].kind, TokenKind::Identifier(value) if value == "Document1"));
        assert!(matches!(&tokens[2].kind, TokenKind::BlockDoc(value) if value == " hi "));
        assert!(
            tokens
                .iter()
                .any(|token| matches!(&token.kind, TokenKind::String(value) if value == "uncl"))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Question))
        );
    }

    #[test]
    fn collects_leading_comment_trivia_on_the_next_token() {
        let tokens = lex(
            "// header note\npackage Demo {\n    /* engine block */\n    part def Engine;\n}\n",
        )
        .unwrap();

        let package = tokens
            .iter()
            .find(|token| matches!(token.kind, TokenKind::Package))
            .unwrap();
        assert_eq!(package.leading_trivia.len(), 1);
        assert_eq!(package.leading_trivia[0].text, " header note");
        assert_eq!(package.leading_trivia[0].kind, CommentKind::Line);
        assert!(package.leading_trivia[0].own_line);

        let part = tokens
            .iter()
            .find(|token| matches!(token.kind, TokenKind::Part))
            .unwrap();
        assert!(part.leading_trivia.is_empty());
        let body = tokens
            .iter()
            .find(|t| matches!(&t.kind, TokenKind::BlockDoc(value) if value == " engine block "))
            .unwrap();
        assert_eq!(body.span.start_line, 3);
    }

    #[test]
    fn marks_trailing_same_line_comments_as_not_own_line() {
        let tokens = lex("part def Engine; // trailing note\npart def Chassis;").unwrap();

        let second_part = tokens
            .iter()
            .filter(|token| matches!(token.kind, TokenKind::Part))
            .nth(1)
            .unwrap();
        assert_eq!(second_part.leading_trivia.len(), 1);
        assert_eq!(second_part.leading_trivia[0].text, " trailing note");
        assert!(!second_part.leading_trivia[0].own_line);
    }

    #[test]
    fn does_not_collect_doc_bodies_or_doc_candidate_blocks_as_trivia() {
        let tokens =
            lex("package Demo { doc /* docs */ comment /* about */ part def Engine; }").unwrap();

        assert!(
            tokens.iter().all(|token| token.leading_trivia.is_empty()),
            "doc and comment-usage bodies must not be captured as trivia"
        );
        assert!(
            tokens.iter().any(
                |token| matches!(&token.kind, TokenKind::BlockDoc(value) if value == " docs ")
            )
        );
        assert!(
            tokens.iter().any(
                |token| matches!(&token.kind, TokenKind::BlockDoc(value) if value == " about ")
            )
        );
    }

    #[test]
    fn treats_line_prefixed_placeholder_comments_as_block_comments() {
        let tokens = lex("attribute x = ( //* ... */ );").unwrap();

        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::LParen))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::RParen))
        );
        assert!(
            tokens
                .iter()
                .any(|token| matches!(token.kind, TokenKind::Semicolon))
        );
    }
    #[test]
    fn scientific_numbers_keep_exponents_and_following_tokens() {
        let tokens = lex("7.2973525693E-3[one] 1e+4 .5E2 2e0 1..3").unwrap();
        let numbers = tokens
            .iter()
            .filter_map(|token| match &token.kind {
                TokenKind::Number(value) => Some(value.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            numbers,
            ["7.2973525693E-3", "1e+4", ".5E2", "2e0", "1", "3"]
        );
        assert!(matches!(tokens[1].kind, TokenKind::LBracket));
        assert!(
            tokens
                .windows(2)
                .any(|pair| matches!(pair[0].kind, TokenKind::Dot)
                    && matches!(pair[1].kind, TokenKind::Dot))
        );
    }

    #[test]
    fn rejects_incomplete_numeric_exponents() {
        for source in ["1e", "1E+", ".5e-", "7.2E[one]"] {
            let error = lex(source).unwrap_err();
            assert!(
                error.message.contains("numeric exponent"),
                "{source}: {error}"
            );
        }
    }
}

#[cfg(test)]
mod language_hidden_contract_tests {
    use super::*;
    fn identifier_terminal(rule: &str, input: &str) -> Option<usize> {
        let length = input.bytes().take_while(u8::is_ascii_alphabetic).count();
        (rule == "ID" && length > 0).then_some(length)
    }
    #[test]
    fn language_terminal_spans_control_identifiers_and_comment_bodies() {
        fn terminal(rule: &str, input: &str) -> Option<usize> {
            match rule {
                "ID" => input.starts_with('a').then_some(1),
                "REGULAR_COMMENT" => input.starts_with("/*body*/").then_some(8),
                _ => None,
            }
        }
        let tokens = lex_with_terminal_matcher("aa/*body*/a", terminal).unwrap();
        assert_eq!(tokens.len(), 5);
        assert!(matches!(&tokens[2].kind, TokenKind::BlockDoc(value) if value == "body"));
        assert_eq!(tokens[3].span.start_col, 11);
        assert!(lex_with_terminal_matcher("b", terminal).is_err());
        assert!(lex_with_terminal_matcher("/*other*/", terminal).is_err());
        for matcher in [
            (|_: &str, _: &str| Some(0)) as TerminalMatcher,
            |_: &str, _: &str| Some(99),
            |_: &str, _: &str| Some(2),
        ] {
            assert!(lex_with_terminal_matcher("aé", matcher).is_err());
        }
    }

    #[test]
    fn rejects_invalid_language_number_ranges() {
        for matcher in [
            (|_: &str| Ok(0)) as NumberMatcher,
            |_: &str| Ok(99),
            |_: &str| Ok(2), // inside the UTF-8 encoding of é
            |_: &str| Err("rejected numeric grammar"),
        ] {
            assert!(
                lex_with_numeric_matcher("1é", identifier_terminal, |_| Ok(None), matcher).is_err()
            );
        }
        let tokens =
            lex_with_numeric_matcher("1x", identifier_terminal, |_| Ok(None), |_| Ok(1)).unwrap();
        assert!(matches!(&tokens[0].kind, TokenKind::Number(value) if value == "1"));
        assert_eq!(tokens[1].span.start_col, 2);
    }

    #[test]
    fn rejects_nonprogress_and_invalid_utf8_hidden_ranges() {
        assert!(
            lex_with_language_matchers("a", identifier_terminal, |_| Ok(Some(HiddenTokenMatch {
                length: 0,
                comment: None
            })))
            .is_err()
        );
        assert!(
            lex_with_language_matchers("é", identifier_terminal, |_| Ok(Some(HiddenTokenMatch {
                length: 1,
                comment: None
            })))
            .is_err()
        );
        assert!(
            lex_with_language_matchers("é", identifier_terminal, |_| Ok(Some(HiddenTokenMatch {
                length: 2,
                comment: Some((CommentKind::Line, 0..1))
            })))
            .is_err()
        );
    }
    #[test]
    fn language_policy_controls_skipping_without_concrete_grammar_names() {
        fn hidden(input: &str) -> Result<Option<HiddenTokenMatch>, &'static str> {
            Ok(input.starts_with('~').then_some(HiddenTokenMatch {
                length: 1,
                comment: None,
            }))
        }
        let tokens = lex_with_language_matchers("a~b", identifier_terminal, hidden).unwrap();
        assert_eq!(tokens.len(), 3);
        assert!(matches!(&tokens[1].kind, TokenKind::Identifier(name) if name == "b"));
        assert_eq!(tokens[1].span.start_col, 3);
    }
}
