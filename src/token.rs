#[derive(Clone, Debug, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // ONE OR TWO CHARACTER TOKENS.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // LITERALS.
    IDENTIFIER,
    STRING,
    NUMBER,

    // KEYWORDS.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: usize,
}

impl Token {
    pub fn to_string(&self) -> String {
        let literal = if self.literal.is_empty() {
            "null".to_string()
        } else {
            self.literal.clone()
        };
        format!("{:?} {} {}", self.token_type, self.lexeme, literal)
    }
}
