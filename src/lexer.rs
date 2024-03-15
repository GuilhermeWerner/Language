use std::iter::Peekable;
use std::ops::DerefMut;
use std::str::Chars;

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Const,
    Else,
    For,
    Function,
    If,
    Import,
    Return,
    Var,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    CloseBrace,   // "}"
    CloseBracket, // "]"
    CloseParen,   // ")"
    Comma,        // ","
    Eof,
    Ident(String),
    Keyword(Keyword),
    LineComment,
    Number(f64),
    Op(char),
    OpenBrace,   // "{"
    OpenBracket, // "["
    OpenParen,   // "("
    Semi,        // ";"
    SemiColon,
}

/// Defines an error encountered by the `Lexer`.
pub struct LexerError {
    pub error: &'static str,
    pub index: usize,
}

impl LexerError {
    #[allow(unused)]
    pub fn new(msg: &'static str) -> LexerError {
        LexerError {
            error: msg,
            index: 0,
        }
    }

    #[allow(unused)]
    pub fn with_index(msg: &'static str, index: usize) -> LexerError {
        LexerError { error: msg, index }
    }
}

/// Defines the result of a lexing operation; namely a
/// `Token` on success, or a `LexerError` on failure.
pub type LexerResult = Result<TokenKind, LexerError>;

pub struct Lexer<'a> {
    input: &'a str,
    chars: Box<Peekable<Chars<'a>>>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer`, given its source `input`.
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input,
            chars: Box::new(input.chars().peekable()),
            pos: 0,
        }
    }

    /// Lexes and returns the next `Token` from the source code.
    pub fn lex(&mut self) -> LexerResult {
        let chars = self.chars.deref_mut();
        let src = self.input;

        let mut pos = self.pos;

        // Skip whitespaces
        loop {
            // Note: the following lines are in their own scope to
            // limit how long 'chars' is borrowed, and in order to allow
            // it to be borrowed again in the loop by 'chars.next()'.
            {
                let ch = chars.peek();

                if ch.is_none() {
                    self.pos = pos;

                    return Ok(TokenKind::Eof);
                }

                if !ch.unwrap().is_whitespace() {
                    break;
                }
            }

            chars.next();
            pos += 1;
        }

        let start = pos;
        let next = chars.next();

        if next.is_none() {
            return Ok(TokenKind::Eof);
        }

        pos += 1;

        // Actually get the next token.
        let result = match next.unwrap() {
            ';' => Ok(TokenKind::Semi),
            ',' => Ok(TokenKind::Comma),
            '(' => Ok(TokenKind::OpenParen),
            ')' => Ok(TokenKind::CloseParen),
            '{' => Ok(TokenKind::OpenBrace),
            '}' => Ok(TokenKind::CloseBrace),
            '[' => Ok(TokenKind::OpenBracket),
            ']' => Ok(TokenKind::CloseBracket),

            '#' => {
                // Comment
                loop {
                    let ch = chars.next();
                    pos += 1;

                    if ch == Some('\n') {
                        break;
                    }
                }

                Ok(TokenKind::LineComment)
            }

            '.' | '0'..='9' => {
                // Parse number literal
                loop {
                    let ch = match chars.peek() {
                        Some(ch) => *ch,
                        None => return Ok(TokenKind::Eof),
                    };

                    // Parse float.
                    if ch != '.' && !ch.is_ascii_hexdigit() {
                        break;
                    }

                    chars.next();
                    pos += 1;
                }

                Ok(TokenKind::Number(src[start..pos].parse().unwrap()))
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                // Parse identifier
                loop {
                    let ch = match chars.peek() {
                        Some(ch) => *ch,
                        None => return Ok(TokenKind::Eof),
                    };

                    // A word-like identifier only contains underscores and alphanumeric characters.
                    if ch != '_' && !ch.is_alphanumeric() {
                        break;
                    }

                    chars.next();
                    pos += 1;
                }

                match &src[start..pos] {
                    "const" => Ok(TokenKind::Keyword(Keyword::Const)),
                    "else" => Ok(TokenKind::Keyword(Keyword::Else)),
                    "for" => Ok(TokenKind::Keyword(Keyword::For)),
                    "function" => Ok(TokenKind::Keyword(Keyword::Function)),
                    "if" => Ok(TokenKind::Keyword(Keyword::If)),
                    "import" => Ok(TokenKind::Keyword(Keyword::Import)),
                    "return" => Ok(TokenKind::Keyword(Keyword::Return)),
                    "var" => Ok(TokenKind::Keyword(Keyword::Var)),
                    ident => Ok(TokenKind::Ident(ident.to_string())),
                }
            }

            op => {
                // Parse operator
                Ok(TokenKind::Op(op))
            }
        };

        // Update stored position, and return
        self.pos = pos;

        result
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = TokenKind;

    /// Lexes the next `Token` and returns it.
    /// On EOF or failure, `None` will be returned.
    fn next(&mut self) -> Option<Self::Item> {
        match self.lex() {
            Ok(TokenKind::Eof) | Err(_) => None,
            Ok(token) => Some(token),
        }
    }
}
