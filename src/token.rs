use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    Number(f32),
    Identifier(String),

    If,
    Then,
    Else,
    End,

    Eq,
    NEq,
    IsEq,
    Gt,
    Lt,
    GtEq,
    LtEq,

    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    Differentiate,

    Comma,
    Belongs,
    Colon,
    LParen,
    RParen,
    LCurly,
    RCurly,
    VLine,
}

impl Token {
    pub fn new(token: String) -> Self {
        match token.as_str().trim() {
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "end" => Token::End,

            "=" => Token::Eq,
            "!=" => Token::NEq,
            "==" => Token::IsEq,
            ">" => Token::Gt,
            "<" => Token::Lt,
            ">=" => Token::GtEq,
            "<=" => Token::LtEq,

            "+" => Token::Add,
            "-" => Token::Sub,
            "*" => Token::Mul,
            "/" => Token::Div,
            "^" => Token::Pow,
            "%" => Token::Mod,
            "`" => Token::Differentiate,

            "," => Token::Comma,
            "E" => Token::Belongs,
            ":" => Token::Colon,
            "(" => Token::LParen,
            ")" => Token::RParen,
            "{" => Token::LCurly,
            "}" => Token::RCurly,
            "|" => Token::VLine,
            _ => {
                if token.chars().all(|a| a.is_ascii_digit() || a=='.') {
                    Token::Number(token.parse::<f32>().unwrap())
                } else {
                    Token::Identifier(token)
                }
            }
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Token::Number(n) => n.to_string(),
            Token::Identifier(ident) => ident.to_string(),
            Token::If => "if".to_string(),
            Token::Then => "then".to_string(),
            Token::Else => "else".to_string(),
            Token::End => "end".to_string(),
            Token::Eq => "=".to_string(),
            Token::NEq => "!=".to_string(),
            Token::IsEq => "==".to_string(),
            Token::Gt => ">".to_string(),
            Token::Lt => "<".to_string(),
            Token::GtEq => ">=".to_string(),
            Token::LtEq => "<=".to_string(),
            Token::Add => "+".to_string(),
            Token::Sub => "-".to_string(),
            Token::Mul => "*".to_string(),
            Token::Div => "/".to_string(),
            Token::Pow => "^".to_string(),
            Token::Mod => "%".to_string(),
            Token::Differentiate => "`".to_string(),
            Token::Comma => ",".to_string(),
            Token::Belongs => "E".to_string(),
            Token::Colon => ":".to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::LCurly => "{".to_string(),
            Token::RCurly => "}".to_string(),
            Token::VLine => "|".to_string(),
        })
    }
}