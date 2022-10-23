#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Symbol(Symbol),
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let, // gr
    If, // grr
    Else, // grrrr
    Loop, // grrgrr
    Fn, // grrbarkgrr
    Return, // grrbark
    Break, // grrbarkgrrbark
    Continue, // grrbarkgrrbarkgrrbark
    While // grrgrrbarkbark
}

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Add, // yawn
    Minus, // sits
    Divide, // whimpers
    Multiply, // humps
    Assign, // howls
    IsEqualTo, // shits
    NotEqualTo, // pees
    Semicolon, // \n
    CurlyBraceOpen, // bites
    CurlyBraceClose, // licks
}

pub fn tokenise(code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut token_as_str: String = "".to_string();

    for char in code.chars() {
        match char {
            ' ' => {
                if token_as_str.len() > 0 {
                    tokens.push(create_token(&token_as_str));
                    token_as_str = "".to_string();
                }
            },

            '\n' => {
                tokens.push(create_token(&token_as_str));
                tokens.push(Token::Symbol(Symbol::Semicolon));
                token_as_str = "".to_string();
            },

            '*' => {
                if token_as_str.len() > 0 {
                    tokens.push(create_token(&token_as_str));
                    token_as_str = "".to_string();
                }
            }

            _ => {
                token_as_str.push(char);
            }

        }
    }

    return tokens
}

pub fn create_token(token_as_str: &str) -> Token {
    return match token_as_str {
        "gr" => Token::Keyword(Keyword::Let),
        "grr" => Token::Keyword(Keyword::If),
        "grrrr" => Token::Keyword(Keyword::Else),
        "grrgrr" => Token::Keyword(Keyword::Loop),
        "grrbarkgrr" => Token::Keyword(Keyword::Fn),
        "grrbark" => Token::Keyword(Keyword::Return),
        "grrbarkgrrbark" => Token::Keyword(Keyword::Break),
        "grrbarkgrrbarkgrrbark" => Token::Keyword(Keyword::Continue),
        "grrgrrbarkbark" => Token::Keyword(Keyword::While),

        "yawn" => Token::Symbol(Symbol::Add),
        "sits" => Token::Symbol(Symbol::Minus),
        "whimpers" => Token::Symbol(Symbol::Divide),
        "humps" => Token::Symbol(Symbol::Multiply),
        "howls" => Token::Symbol(Symbol::Assign),
        "shits" => Token::Symbol(Symbol::IsEqualTo),
        "pees" => Token::Symbol(Symbol::NotEqualTo),
        "\n" => Token::Symbol(Symbol::Semicolon),
        "bites" => Token::Symbol(Symbol::CurlyBraceOpen),
        "licks" => Token::Symbol(Symbol::CurlyBraceClose),
        
        token_as_str => {
            if is_int(token_as_str) {
                Token::Int(token_as_str.parse::<i32>().unwrap())

            } else if is_float(token_as_str) {
                Token::Float(token_as_str.parse::<f32>().unwrap())
            
            } else if is_bool(token_as_str) {
                Token::Bool(token_as_str.parse::<bool>().unwrap())

            } else if is_yap(token_as_str) {
                Token::Int(count_yaps(token_as_str))

            } else {
                Token::Identifier(token_as_str.to_string())
            }
        }
    }
}

pub fn is_int(token_as_str: &str) -> bool {
    return token_as_str.parse::<i32>().is_ok();
}

pub fn is_float(token_as_str: &str) -> bool {
    return token_as_str.parse::<f32>().is_ok();
}

pub fn is_bool(token_as_str: &str) -> bool {
    return token_as_str.parse::<bool>().is_ok();
}

pub fn is_yap(token_as_str: &str) -> bool {
    return token_as_str.starts_with("yap");
}

pub fn count_yaps(token_as_str: &str) -> i32 {
    let mut num_y = 0;
    let mut num_a = 0;
    let mut num_p = 0;

    for char in token_as_str.chars() {
        match char {
            'y' => num_y += 1,
            'a' => num_a += 1,
            'p' => num_p += 1,
            _ => {}
        }
    }

    if (num_y == num_a) && (num_a == num_p) {
        return num_y;
    } else {
        return 0;
    }
}