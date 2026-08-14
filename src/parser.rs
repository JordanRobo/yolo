use super::ast::{Expr, Program, Stmt, Tone, Vibe};
use super::lexer::{SpannedToken, Token};

pub struct Parser {
    tokens: Vec<SpannedToken>,
    pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse_program(mut self) -> Result<Program, ParseError> {
        let mut stmts = Vec::new();

        while !self.check(&Token::Eof) {
            stmts.push(self.parse_statement()?);
        }

        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        let line = self.peek_line();
    
        if self.check(&Token::Maybe) {
            return self.parse_manifest(true, line);
        }
        if self.check(&Token::Manifest) {
            return self.parse_manifest(false, line);
        }
        if let Token::Ident(_) = self.peek_token() {
            if self.check_ahead(1, &Token::As) {
                return self.parse_reassign(line);
            }
        }
    
        let expr = self.parse_expr()?;
        let tone = self.expect_terminator()?;
        Ok(Stmt::Expr { expr, tone, line })
    }

    fn parse_manifest(&mut self, mutable: bool, line: usize) -> Result<Stmt, ParseError> {
        if mutable { self.expect(&Token::Maybe)?; }
        self.expect(&Token::Manifest)?;
        let name = self.expect_ident()?;
        self.expect(&Token::As)?;
        let value = self.parse_expr()?;
        let tone = self.expect_terminator()?;
        Ok(Stmt::Manifest { name, value, mutable, tone, line })
    }

    fn parse_reassign(&mut self, line: usize) -> Result<Stmt, ParseError> {
        let name = self.expect_ident()?;
        self.expect(&Token::As)?;
        let value = self.parse_expr()?;
        let tone = self.expect_terminator()?;
        Ok(Stmt::Reassign { name, value, tone, line })
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        match self.peek_token().clone() {
            Token::StringLit(s) => {
                self.advance();
                Ok(Expr::StringLit(s))
            }
            Token::Chill => { self.advance(); Ok(Expr::VibeLit(Vibe::Chill)) }
            Token::Stressed => { self.advance(); Ok(Expr::VibeLit(Vibe::Stressed)) }
            Token::Unhinged => { self.advance(); Ok(Expr::VibeLit(Vibe::Unhinged)) }
            Token::Based => { self.advance(); Ok(Expr::VibeLit(Vibe::Based)) }
            Token::Ident(name) => {
                self.advance();
                if self.check(&Token::LParen) {
                    self.parse_call(name)
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            other => Err(self.error(format!("expected an expression, found {:?}", other))),
        }
    }

    fn parse_call(&mut self, callee: String) -> Result<Expr, ParseError> {
        self.expect(&Token::LParen)?;
        let mut args = Vec::new();

        if !self.check(&Token::RParen) {
            args.push(self.parse_expr()?);
            while self.check(&Token::Comma) {
                self.advance();
                args.push(self.parse_expr()?);
            }
        }

        self.expect(&Token::RParen)?;
        Ok(Expr::Call { callee, args })
    }

    fn expect_terminator(&mut self) -> Result<Tone, ParseError> {
        match self.peek_token().clone() {
            Token::Terminator(tone) => {
                self.advance();
                Ok(tone)
            }
            other => Err(self.error(format!(
                "statement needs a vibe to end on - expected . ! ? or -, found {:?}",
                other
            ))),
        }
    }

    // --- token helpers ---

    fn peek_token(&self) -> &Token {
        &self.tokens[self.pos].token
    }

    fn peek_line(&self) -> usize {
        self.tokens[self.pos].line
    }

    fn check(&self, t: &Token) -> bool {
        self.peek_token() == t
    }

    fn check_ahead(&self, offset: usize, t: &Token) -> bool {
        self.tokens
            .get(self.pos + offset)
            .map(|st| &st.token == t)
            .unwrap_or(false)
    }

    fn advance(&mut self) -> &Token {
        let t = &self.tokens[self.pos].token;
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        t
    }

    fn expect(&mut self, t: &Token) -> Result<(), ParseError> {
        if self.check(t) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(format!("expected {:?}, found {:?}", t, self.peek_token())))
        }
    }

    fn expect_ident(&mut self) -> Result<String, ParseError> {
        match self.peek_token().clone() {
            Token::Ident(name) => {
                self.advance();
                Ok(name)
            }
            other => Err(self.error(format!("expected identifier, found {:?}", other))),
        }
    }

    fn error(&self, message: String) -> ParseError {
        ParseError { message, line: self.peek_line() }
    }
}