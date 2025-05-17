#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(u32),
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LEFTPAREN,
    RIGHTPAREN,
    MINUS,
    PLUS,
    SLASH,
    STAR,
    OR,
    AND,
    MOD,

    BANG,
    BANGEQUAL,
    STRINGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESSER,
    LESSEREQUAL,

    IDENTIFIER,
    STRING,
    NUMBER,
    TRUE,
    FALSE,

    COND,
    IF,
    ELSE,
    FUN,
    LIST,
}

impl Clone for TokenType {
    fn clone(&self) -> Self {
        match self {
            Self::LEFTPAREN => Self::LEFTPAREN,
            Self::RIGHTPAREN => Self::RIGHTPAREN,
            Self::MINUS => Self::MINUS,
            Self::PLUS => Self::PLUS,
            Self::SLASH => Self::SLASH,
            Self::STAR => Self::STAR,
            Self::OR => Self::OR,
            Self::AND => Self::AND,
            Self::MOD => Self::MOD,
            Self::BANG => Self::BANG,
            Self::BANGEQUAL => Self::BANGEQUAL,
            Self::STRINGEQUAL => Self::STRINGEQUAL,
            Self::EQUAL => Self::EQUAL,
            Self::EQUALEQUAL => Self::EQUALEQUAL,
            Self::GREATER => Self::GREATER,
            Self::GREATEREQUAL => Self::GREATEREQUAL,
            Self::LESSER => Self::LESSER,
            Self::LESSEREQUAL => Self::LESSEREQUAL,
            Self::IDENTIFIER => Self::IDENTIFIER,
            Self::STRING => Self::STRING,
            Self::NUMBER => Self::NUMBER,
            Self::TRUE => Self::TRUE,
            Self::FALSE => Self::FALSE,
            Self::COND => Self::COND,
            Self::IF => Self::IF,
            Self::ELSE => Self::ELSE,
            Self::FUN => Self::FUN,
            Self::LIST => Self::LIST,
        }
    }
}

impl Copy for TokenType {}
