use super::ast::Tone;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Manifest,
    Maybe,
    As,

    // Vibes
    Chill,
    Stressed,
    Unhinged,
    Based,

    // Literals / names
    Ident(String),
    StringLit(String),

    // Punctuation
    LParen,
    RParen,
    Comma,
    Terminator(Tone), // replaces Semicolon

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    pub line: usize,
}

pub struct Lexer<'a> {
    source: Vec<char>,
    pos: usize,
    line: usize,
    _marker: std::marker::PhantomData<&'a str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexError {
    pub message: String,
    pub line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<SpannedToken>, LexError> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments();

            if self.is_at_end() {
                tokens.push(SpannedToken { token: Token::Eof, line: self.line });
                break;
            }

            let start_line = self.line;
            let c = self.advance();

            let token = match c {
                '(' => Token::LParen,
                ')' => Token::RParen,
                ',' => Token::Comma,
                '.' => {
                    if self.peek() == '.' && self.peek_at(1) == '.' {
                        self.advance(); // consume 2nd '.'
                        self.advance(); // consume 3rd '.'
                        Token::Terminator(Tone::TrailingOff)
                    } else {
                        Token::Terminator(Tone::Plain)
                    }
                }
                '!' => Token::Terminator(Tone::Urgent),
                '?' => Token::Terminator(Tone::Tentative),
                '-' => Token::Terminator(Tone::AgentHandoff),
                '-' if self.peek() == '-' => {
                    self.advance(); // consume 2nd '-'
                    Token::Terminator(Tone::Impersonator)
                },
                '"' => self.string_literal()?,
                c if c.is_ascii_alphabetic() || c == '_' => self.identifier_or_keyword(c),
                c => {
                    return Err(LexError {
                        message: format!("unexpected character '{}'", c),
                        line: start_line,
                    })
                }
            };

            tokens.push(SpannedToken { token, line: start_line });
        }

        Ok(tokens)
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn peek(&self) -> char {
        *self.source.get(self.pos).unwrap_or(&'\0')
    }

    fn peek_at(&self, offset: usize) -> char {
        *self.source.get(self.pos + offset).unwrap_or(&'\0')
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.pos];
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
        }
        c
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\t' | '\r' | '\n' => {
                    self.advance();
                }
                '#' => {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    fn string_literal(&mut self) -> Result<Token, LexError> {
        let start_line = self.line;
        let mut value = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            value.push(self.advance());
        }

        if self.is_at_end() {
            return Err(LexError {
                message: "unterminated string literal".to_string(),
                line: start_line,
            });
        }

        self.advance(); // consume closing quote
        Ok(Token::StringLit(value))
    }

    fn identifier_or_keyword(&mut self, first: char) -> Token {
        let mut text = String::new();
        text.push(first);

        while !self.is_at_end() && (self.peek().is_ascii_alphanumeric() || self.peek() == '_') {
            text.push(self.advance());
        }

        match text.as_str() {
            "manifest" => Token::Manifest,
            "maybe" => Token::Maybe,
            "as" => Token::As,
            "chill" => Token::Chill,
            "stressed" => Token::Stressed,
            "unhinged" => Token::Unhinged,
            "based" => Token::Based,
            _ => Token::Ident(text),
        }
    }
}