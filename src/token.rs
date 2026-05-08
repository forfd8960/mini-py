#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    If, Else, Def, Return, While, Let,

    // Identifiers and Literals
    Ident(String),       // e.g., variable or function names [5]
    Int(i64),            // e.g., 42 [5]
    Float(f64),          // e.g., 3.14 [5]
    StringLit(String),   // e.g., "hello" [5]

    // Operators
    Plus, Minus, Star, Slash,  // +, -, *, / [5]
    Assign,                    // = 
    EqualEqual,                // == [5]
    NotEqual,                  // !=
    Less, LessEqual,           // <, <=
    Greater, GreaterEqual,     // >, >=
    Not,                       // ! or not

    // Delimiters
    LParen, RParen,      // ( and ) [5]
    Comma,               // , [5]
    Colon,               // : [5]
    Semicolon,           // ; 

    // Block Delimiters (Replaces Indent/Dedent)
    LBrace,              // {
    RBrace,              // }
    
    // End of File
    EOF,
}