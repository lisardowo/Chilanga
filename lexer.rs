use crate::lexer::TokenType::Operator;

const SYMBOLS: &str = "<>+-*/%=!:";

const PUNCTUATORS: &str = "(){}[];,.";

const KEYWORDS: &[&str; 8] = &[
    "char", "int", "else", "float", "for", "if", "return", "void",
];

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword(String),
    Identifier(String),
    Literal(LiteralValue),
    Operator(String),
    Symbol(char),
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Normal,
    InString { start: (usize, usize) },
    InChar { start: (usize, usize) },
    InBlockComment { start: (usize, usize) },
    InLineComment,
}

pub struct Lexer {
    tokens: Vec<Token>,
    buffer: String,
    bufferStart: (usize, usize),
    state: State,
    literal: String,
    prev: Option<char>,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            tokens: Vec::new(),
            buffer: String::new(),
            bufferStart: (1, 1),
            state: State::Normal,
            literal: String::new(),
            prev: None,
        }
    }

    pub fn tokenize(mut self, source: &str) -> Vec<Token> {
        let mut chars = source.chars().peekable();
        let (mut line, mut column) = (1usize, 1usize);

        while let Some(ch) = chars.next() {
            let pos = (line, column);
            let next = chars.peek().copied();

            self.step(ch, pos, next, &mut chars, &mut line, &mut column);
        }

        self.eof();
        self.tokens
    }

    fn step(
        &mut self,
        ch: char,
        pos: (usize, usize),
        next: Option<char>,
        chars: &mut std::iter::Peekable<std::str::Chars>,
        line: &mut usize,
        column: &mut usize,
    ) {
        match self.state {
            State::Normal => self.normal(ch, pos, next, chars),
            State::InString { start } => self.inString(ch, start),
            State::InChar { start } => self.inChar(ch, start),
            State::InBlockComment { .. } => self.inBlockComment(ch),
            State::InLineComment => self.inLineComment(ch),
        }

        if ch == '\n' {
            *line += 1;
            *column = 1;
        } else {
            *column += 1;
        }
    }

    fn normal(
        &mut self,
        ch: char,
        pos: (usize, usize),
        next: Option<char>,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) {
        if ch == '/' && next == Some('/') {
            self.flush();
            self.state = State::InLineComment;
            chars.next();
            self.consumeSkipped('/');
            return;
        }

        if ch == '/' && next == Some('*') {
            self.flush();
            self.state = State::InBlockComment { start: pos };
            chars.next();
            self.consumeSkipped('*');
            return;
        }

        if ch == '"' {
            self.flush();
            self.state = State::InString { start: pos };
            self.literal.clear();
            return;
        }

        if ch == '\'' {
            self.flush();
            self.state = State::InChar { start: pos };
            self.literal.clear();
            return;
        }

        if SYMBOLS.contains(ch) || PUNCTUATORS.contains(ch) {
            self.flush();
            self.emit(ch.to_string(), pos);

            return;
        }

        if ch.is_whitespace() {
            self.flush();
            return;
        }

        if self.buffer.is_empty() {
            self.bufferStart = pos;
        }
        self.buffer.push(ch);
    }

    fn inString(&mut self, ch: char, start: (usize, usize)) {
        match ch {
            '"' if self.prev != Some('\\') => {
                let value = std::mem::take(&mut self.literal);
                self.emitLiteral(LiteralValue::String(value), start);
                self.state = State::Normal;
            }
            '\n' => {
                self.literal.clear();
                self.error("unclosed char", start);
                self.state = State::Normal;
            }
            '\\' if self.prev != Some('\\') => {
                self.literal.push('\\');
            }
            _ => {
                if self.literal.ends_with('\\') && self.prev == Some('\\') {
                    self.literal.pop();
                    self.literal.push(unescape(ch));
                } else {
                    self.literal.push(ch);
                }
            }
        }
        self.prev = Some(ch);
    }

    fn inChar(&mut self, ch: char, start: (usize, usize)) {
        match ch {
            '\'' if self.prev != Some('\\') => {
                let mut chars = self.literal.chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) => {
                        self.emitLiteral(LiteralValue::Char(c), start);
                    }
                    _ => self.error("Invalid char", start),
                }
                self.literal.clear();
                self.state = State::Normal;
            }
            _ => {
                if self.prev == Some('\\') && self.literal.ends_with('\\') {
                    self.literal.pop();
                    self.literal.push(unescape(ch));
                } else {
                    self.literal.push(ch);
                }
            }
        }
        self.prev = Some(ch);
    }

    fn inBlockComment(&mut self, ch: char) {
        if self.prev == Some('*') && ch == '/' {
            self.state = State::Normal;
            self.prev = None;
            return;
        }
        self.prev = Some(ch);
    }

    fn inLineComment(&mut self, ch: char) {
        if ch == '\n' {
            self.state = State::Normal;
        }
        self.prev = Some(ch);
    }

    fn eof(&mut self) {
        self.flush();
        match self.state {
            State::Normal | State::InLineComment => {}
            State::InString { start } => {
                self.error("error: string not closed at eof", start);
            }
            State::InChar { start } => {
                self.error("char not closed at eof ", start);
            }
            State::InBlockComment { start } => {
                self.error("coment not closed at eof ", start);
            }
        }
    }

    fn consumeSkipped(&mut self, ch: char) {
        self.prev = Some(ch);
    }

    fn flush(&mut self) {
        if self.buffer.is_empty() {
            return;
        }
        let word = std::mem::take(&mut self.buffer);
        let start = self.bufferStart;
        self.emit(word, start);
    }

    fn emit(&mut self, lexeme: String, (line, column): (usize, usize)) {
        let token_type = classify(&lexeme);
        self.tokens.push(Token {
            lexeme,
            token_type,
            line,
            column,
        });
    }

    fn emitLiteral(&mut self, value: LiteralValue, (line, column): (usize, usize)) {
        self.tokens.push(Token {
            lexeme: String::new(),
            token_type: TokenType::Literal(value),
            line,
            column,
        });
    }

    fn error(&mut self, message: &str, (line, column): (usize, usize)) {
        self.tokens.push(Token {
            lexeme: String::new(),
            token_type: TokenType::Error(message.to_string()),
            line,
            column,
        });
    }
}

fn unescape(c: char) -> char {
    match c {
        'n' => '\n',
        't' => '\t',
        'r' => '\r',
        '0' => '\0',
        '\\' => '\\',
        '"' => '"',
        '\'' => '\'',
        other => other,
    }
}

fn classify(lexeme: &str) -> TokenType {
    if PUNCTUATORS.contains(&lexeme) {
        if lexeme.len() == 1 && lexeme.is_ascii() {
            let c = lexeme.as_bytes()[0] as char;
            return TokenType::Symbol(c);
        }
        return TokenType::Error("PUNCTUATOR not valid".to_string()); // not proper way to return an error but lets hope
    }

    if KEYWORDS.contains(&lexeme) {
        return TokenType::Keyword(lexeme.to_string());
    }
    if SYMBOLS.contains(&lexeme) {
        return TokenType::Operator(lexeme.to_string());
    }

    if lexeme.len() == 1 && "(){}[];,.".contains(lexeme) {
        return TokenType::Symbol(lexeme.chars().next().unwrap());
    }

    if lexeme.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
        if lexeme
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return TokenType::Identifier(lexeme.to_string());
        }
        return TokenType::Error(format!("unvalid identifier `{lexeme}`"));
    }

    if let Ok(i) = lexeme.parse::<i64>() {
        return TokenType::Literal(LiteralValue::Integer(i));
    }
    if let Ok(f) = lexeme.parse::<f64>() {
        return TokenType::Literal(LiteralValue::Float(f));
    }

    TokenType::Error(format!("lexeme not recognized`{lexeme}`"))
}
