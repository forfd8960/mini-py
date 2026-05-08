# Implement Mini Python Part 0: Token Types and AST Nodes

## Token types and the Abstract Syntax Tree (AST) node

To build a mini interpreter that uses curly braces `{` and `}` instead of indentation to delimit blocks of code, you can significantly simplify your lexical analysis phase. Unlike Python, which requires maintaining a stateful stack of indentation levels to emit synthetic `INDENT` and `DEDENT` tokens (the "off-side rule"), your lexer only needs to emit tokens exactly as it reads the brace characters. 

Here is the complete Rust implementation for the Token types and the Abstract Syntax Tree (AST) node types required for this language.

### 1. Token Types

Tokens represent the smallest meaningful units of your source code. By using curly braces, we replace structural whitespace tokens with explicit `LBrace` (`{`) and `RBrace` (`}`) tokens.

```rust
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    If, Else, Def, Return, While, Let,

    // Identifiers and Literals
    Ident(String),       // e.g., variable or function names
    Int(i64),            // e.g., 42
    Float(f64),          // e.g., 3.14
    StringLit(String),   // e.g., "hello"

    // Operators
    Plus, Minus, Star, Slash,  // +, -, *, /
    Assign,                    // = 
    EqualEqual,                // ==
    NotEqual,                  // !=
    Less, LessEqual,           // <, <=
    Greater, GreaterEqual,     // >, >=
    Not,                       // ! or not

    // Delimiters
    LParen, RParen,      // ( and )
    Comma,               // ,
    Colon,               // :
    Semicolon,           // ; 

    // Block Delimiters (Replaces Indent/Dedent)
    LBrace,              // {
    RBrace,              // }
    
    // End of File
    EOF,
}
```

### 2. AST Node Types

The Abstract Syntax Tree is a recursive data structure representing the logical hierarchy of the program. It is generally split into **Expressions** (computations that return values) and **Statements** (actions that don't return values). 

Because Rust requires the size of an `enum` variant to be known at compile time, any recursive fields (like an expression containing another expression) must be wrapped in a `Box<T>` to store them on the heap. 

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add, Sub, Mul, Div, Eq, Neq, Lt, Gt, Lte, Gte,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Represents binary operations like `x + y`
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    // Represents operations like `-x` or `!y`
    Unary {
        op: Operator,
        right: Box<Expr>,
    },
    // Represents literals like numbers or strings
    Constant(Token),
    // Represents variable lookups
    Variable(String),
    // Represents function calls, storing the callee and arguments
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // Represents a standalone expression, like a function call
    ExprStmt(Expr),
    
    // Represents a variable assignment containing a target name and value
    Assign {
        target: String,
        value: Expr,
    },
    
    // Represents a `{ ... }` code block containing a sequence of statements
    Block(Vec<Stmt>),
    
    // Represents a conditional block. The branches are stored as Blocks
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    
    // Represents a while loop
    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
    
    // Represents a function definition, storing its name, parameters, and body
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<Stmt>, // Will hold a Stmt::Block
    },
    
    // Represents a return statement
    Return(Option<Expr>),
}
```

### How the Parser Uses These Types

When your parser encounters an `LBrace` (`{`) token, it will start collecting statements into a `Vec<Stmt>` until it encounters the matching `RBrace` (`}`) token. Once the matching closing brace is found, it wraps that sequence of statements in a `Stmt::Block(Vec<Stmt>)` node. This `Block` node can then be seamlessly attached to the `body` field of a `FunctionDef` or the `then_branch` of an `If` statement.

## EBNF grammar for Mini Py

To build the grammar for your mini-Python interpreter, we will combine the Abstract Syntax Tree (AST) node structures described in the architectural design of a Python interpreter with the custom `{` and `}` block delimiters you established in our previous conversation. 

While a standard Python lexer relies on the "off-side rule" to generate synthetic `INDENT` and `DEDENT` tokens based on whitespace, your language design bypasses this complexity. Since we are no longer using newlines and indentation to determine the end of statements and blocks, this grammar introduces semicolons (`;`) to terminate standalone statements and uses curly braces to define blocks.

Here is the Extended Backus-Naur Form (EBNF) grammar that maps directly to your token and AST types:

### EBNF Grammar for Brace-Delimited Mini-Python

```ebnf
(* 1. Program Structure *)
program        ::= statement* EOF

(* 2. Statements (Actions that do not return values) *)
statement      ::= expr_stmt 
                 | assign_stmt 
                 | if_stmt 
                 | while_stmt 
                 | func_def 
                 | return_stmt

expr_stmt      ::= expression ";"
assign_stmt    ::= ( "let" )? IDENTIFIER "=" expression ";"
block          ::= "{" statement* "}"

if_stmt        ::= "if" expression block ( "else" block )?
while_stmt     ::= "while" expression block
func_def       ::= "def" IDENTIFIER "(" parameters? ")" block
return_stmt    ::= "return" expression? ";"

(* Helper rules for sequences *)
parameters     ::= IDENTIFIER ( "," IDENTIFIER )*
arguments      ::= expression ( "," expression )*

(* 3. Expressions (Computations that return values) *)
expression     ::= equality

equality       ::= comparison ( ( "==" | "!=" ) comparison )*
comparison     ::= term ( ( "<" | "<=" | ">" | ">=" ) term )*
term           ::= factor ( ( "+" | "-" ) factor )*
factor         ::= unary ( ( "*" | "/" ) unary )*

unary          ::= ( "-" | "!" ) unary 
                 | call

call           ::= primary ( "(" arguments? ")" )*

primary        ::= IDENTIFIER 
                 | INT 
                 | FLOAT 
                 | STRING_LIT 
                 | "(" expression ")"
```

### How this Grammar maps to your implementation

**Statements vs. Expressions:**

The grammar rigidly separates **Statements** from **Expressions**, which aligns with how Python evaluates syntax. Statements handle actions like variable assignments (`Assign`), conditionals (`If`), loops, and function definitions (`FunctionDef`), while Expressions handle computable operations like binary math (`BinOp`), logic (`UnaryOp`), and variable lookups.

**Parsing Statements (Recursive Descent):**

For the high-level program structure, your parser will use the **recursive descent** technique. When the parser encounters a keyword token like `def` or `if`, it will branch into the corresponding parsing function (e.g., `parse_function_def` or `parse_if_statement`). Because you are using `{` and `}` instead of Python's `INDENT` and `DEDENT` tokens, whenever a statement expects a body of code (like the inside of a `while` loop), it simply consumes an `LBrace` token, loops over `statement` rules until it hits an `RBrace` token, and packages them into a `Stmt::Block`.

**Parsing Expressions (Operator Precedence):**

Notice how the expression rules (`equality` -> `comparison` -> `term` -> `factor` -> `unary` -> `call` -> `primary`) are nested. This strict nesting natively encodes operator precedence (e.g., multiplication binds tighter than addition). 

While you can parse this using deep recursive functions, standard interpreter designs often use **Pratt Parsing** (Top-Down Operator Precedence parsing) to handle expressions more efficiently. Pratt parsing uses numerical "binding powers" for tokens (for example, `*` gets a higher binding power than `+`) to figure out how tightly operators cling to their surrounding expressions, allowing you to parse complex chains like `-x + y * z()` cleanly without needing a massive stack of nested function calls.