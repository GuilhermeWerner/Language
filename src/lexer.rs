use std::iter::Peekable;
use std::ops::DerefMut;
use std::str::Chars;

/// Represents a primitive syntax token.
#[derive(Debug, Clone)]
pub enum Token {
    Comma,
    Comment,
    Const,
    Else,
    EOF,
    For,
    Function,
    Ident(String),
    If,
    Import,
    LParen,
    Number(f64),
    Op(char),
    Return,
    RParen,
    SemiColon,
    Var,
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
pub type LexerResult = Result<Token, LexerError>;

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

                    return Ok(Token::EOF);
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
            return Ok(Token::EOF);
        }

        pos += 1;

        // Actually get the next token.
        let result = match next.unwrap() {
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            ',' => Ok(Token::Comma),
            ';' => Ok(Token::SemiColon),

            '#' => {
                // Comment
                loop {
                    let ch = chars.next();
                    pos += 1;

                    if ch == Some('\n') {
                        break;
                    }
                }

                Ok(Token::Comment)
            }

            '.' | '0'..='9' => {
                // Parse number literal
                loop {
                    let ch = match chars.peek() {
                        Some(ch) => *ch,
                        None => return Ok(Token::EOF),
                    };

                    // Parse float.
                    if ch != '.' && !ch.is_ascii_hexdigit() {
                        break;
                    }

                    chars.next();
                    pos += 1;
                }

                Ok(Token::Number(src[start..pos].parse().unwrap()))
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                // Parse identifier
                loop {
                    let ch = match chars.peek() {
                        Some(ch) => *ch,
                        None => return Ok(Token::EOF),
                    };

                    // A word-like identifier only contains underscores and alphanumeric characters.
                    if ch != '_' && !ch.is_alphanumeric() {
                        break;
                    }

                    chars.next();
                    pos += 1;
                }

                match &src[start..pos] {
                    "function" => Ok(Token::Function),
                    "import" => Ok(Token::Import),
                    "if" => Ok(Token::If),
                    "else" => Ok(Token::Else),
                    "for" => Ok(Token::For),
                    "var" => Ok(Token::Var),
                    "return" => Ok(Token::Return),
                    "const" => Ok(Token::Const),

                    ident => Ok(Token::Ident(ident.to_string())),
                }
            }

            op => {
                // Parse operator
                Ok(Token::Op(op))
            }
        };

        // Update stored position, and return
        self.pos = pos;

        result
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    /// Lexes the next `Token` and returns it.
    /// On EOF or failure, `None` will be returned.
    fn next(&mut self) -> Option<Self::Item> {
        match self.lex() {
            Ok(Token::EOF) | Err(_) => None,
            Ok(token) => Some(token),
        }
    }
}
