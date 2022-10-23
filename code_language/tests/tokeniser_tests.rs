
#[cfg(test)]
mod tests {

    use code_language::tokeniser::{Token, Symbol, Keyword, tokenise};

    #[test]
    fn can_tokenise_simple_variable_declaration() {
        let code = "gr yip howls* yapyapyapyapyap
        ";
        
        let tokens = tokenise(code);
        
        let expected_tokens: Vec<Token> = vec![
            Token::Keyword(Keyword::Let),
            Token::Identifier("yip".to_string()),
            Token::Symbol(Symbol::Assign),
            Token::Int(5),
            Token::Symbol(Symbol::Semicolon),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn can_tokenise_conditionals() {
        let code = "grr yip shits* yapyapyapyapyap
        ";
        
        let tokens = tokenise(code);
        
        let expected_tokens: Vec<Token> = vec![
            Token::Keyword(Keyword::If),
            Token::Identifier("yip".to_string()),
            Token::Symbol(Symbol::IsEqualTo),
            Token::Int(5),
            Token::Symbol(Symbol::Semicolon),
        ];

        assert_eq!(tokens, expected_tokens);
    }
}

