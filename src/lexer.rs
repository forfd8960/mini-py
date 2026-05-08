use std::thread::current;

use crate::{errors::PYError, token::Token};

#[derive(Debug)]
pub struct Lexer {
    pub chars: Vec<char>,
    pub current: usize,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        Self {
            chars: code.chars().collect(),
            current: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, PYError> {
        let mut tokens: Vec<Token> = Vec::with_capacity(1000);
        while !self.is_end() {}
        Ok(vec![])
    }

    pub fn is_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    pub fn peek(&self) -> Option<char> {
        if self.is_end() {
            return None;
        }

        Some(self.chars[self.current])
    }

    pub fn peek_next(&mut self) -> Option<char> {
        if self.is_end() || (self.current + 1) >= self.chars.len() {
            return None;
        }

        Some(self.chars[self.current + 1])
    }

    pub fn advance(&mut self) -> char {
        let ch = self.chars[self.current];
        self.current += 1;
        ch
    }
}
