use std::{any::Any, collections::HashMap, fmt::Display};

use crate::token::{Token, TokenType, TokenType::*};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,

    start: usize,
    current: usize,
    line: u32,

    pub has_error: bool,
}

pub fn build_scanner(source: String) -> Scanner {
    let mut keywords = HashMap::new();
    keywords.insert("and".to_string(), AND);
    keywords.insert("class".to_string(), CLASS);
    keywords.insert("else".to_string(), ELSE);
    keywords.insert("false".to_string(), FALSE);
    keywords.insert("for".to_string(), FOR);
    keywords.insert("fun".to_string(), FUN);
    keywords.insert("if".to_string(), IF);
    keywords.insert("nil".to_string(), NIL);
    keywords.insert("or".to_string(), OR);
    keywords.insert("print".to_string(), PRINT);
    keywords.insert("return".to_string(), RETURN);
    keywords.insert("super".to_string(), SUPER);
    keywords.insert("this".to_string(), THIS);
    keywords.insert("true".to_string(), TRUE);
    keywords.insert("var".to_string(), VAR);
    keywords.insert("while".to_string(), WHILE);

    Scanner {
        source,
        tokens: Vec::new(),
        keywords,
        start: 0,
        current: 0,
        line: 1,
        has_error: false,
    }
}

impl Scanner {
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        // println!("Scanning tokens...");
        // println!("Source: {}", self.source);
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: self.line as usize,
        });

        return self.tokens.clone();
    }

    fn scan_token(&mut self) {
        let c = self.source.as_bytes()[self.current] as char;
        self.current += 1;

        match c {
            '(' => self.add_token(LEFT_PAREN),
            ')' => self.add_token(RIGHT_PAREN),
            '{' => self.add_token(LEFT_BRACE),
            '}' => self.add_token(RIGHT_BRACE),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),
            '!' => {
                if self.match_char('=') {
                    self.add_token(BANG_EQUAL);
                } else {
                    self.add_token(BANG);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(EQUAL_EQUAL);
                } else {
                    self.add_token(EQUAL);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(LESS_EQUAL);
                } else {
                    self.add_token(LESS);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(GREATER_EQUAL);
                } else {
                    self.add_token(GREATER);
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            // literals
            '"' => self.string(),

            'o' => {
                if self.match_char('r') {
                    self.add_token(OR);
                }
            }

            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    // Lox class error here ? how ??
                    eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
                    self.has_error = true;
                }
            }
        }
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self.keywords.get(text).copied().unwrap_or(IDENTIFIER);
        self.add_token(token_type);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // methods for moving through the source
    fn advance(&mut self) {
        self.current += 1;
        // self.source.chars().nth(self.current - 1).unwrap()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.as_bytes()[self.current] as char
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.as_bytes()[self.current + 1] as char
    }

    // methods for token handling
    fn add_token(&mut self, token_type: TokenType) {
        // println!("Adding token: {:?}", token_type);
        self.add_token_literal::<String>(token_type, None)
    }

    fn add_token_literal<T: ToString + Display>(
        &mut self,
        token_type: TokenType,
        literal: Option<T>,
    ) {
        let text = &self.source[self.start..self.current];
        let token = Token {
            token_type,
            lexeme: text.to_string(),
            literal: match literal {
                Some(l) => with_decimal(l),
                None => "".to_string(),
            },
            line: self.line as usize,
        };
        self.tokens.push(token);
    }

    //
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            self.has_error = true;
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_literal(STRING, Some(value));
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        // look for fractional part
        if self.peek() == '.' && is_digit(self.peek_next()) {
            // consume the '.'
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let src = &self.source[self.start..self.current];
        let f: f64 = match src.parse() {
            Ok(f) => f,
            // ERR HANDLE
            Err(_) => panic!("Error parsing number"),
        };
        self.add_token_literal(NUMBER, Some(f + 0.0));
    }
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn with_decimal<T: Display>(value: T) -> String {
    let formatted = format!("{:.1}", value);

    if formatted.ends_with(".0") {
        formatted
    } else {
        return value.to_string();
    }
}

fn is_float<T: Any>(value: &T) -> bool {
    let value_any: &dyn Any = value;
    if value_any.is::<f32>() || value_any.is::<f64>() {
        return true;
    } else {
        return false;
    }
}
