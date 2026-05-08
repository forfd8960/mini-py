use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum PYError {
    #[error("lexer error: {0}")]
    LexerError(LexerError),
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum LexerError {}
