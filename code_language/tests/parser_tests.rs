#[cfg(test)]
mod tests {
use code_language::{tokeniser::{Token, tokenise}, parser::{Statement, parse, Assignment, Value}};

#[test]
fn can_parse_basic_let_statement() {
    let code = "gr yip howls* yapyapyapyapyap
    ";
    let tokens: Vec<Token> = tokenise(code);
    let statements: Vec<Statement> = parse(tokens);
    let expected_statements: Vec<Statement> = vec![
        Statement::Assignment(Assignment { 
            identifier: "yip".to_string(),
            value: Value::Int(5),
         })
    ];

    assert_eq!(statements, expected_statements);
}

#[test]
fn can_parse_basic_if_statement() {
    let code = "grr yip shits* yapyapyapyapyap bites* grrbark yip
    licks*
    ";

    let tokens: Vec<Token> = tokenise(code);
    println!("{:?}", tokens);
    let statements: Vec<Statement> = parse(tokens);
    let expected_statements: Vec<Statement> = vec![
        Statement::Assignment(Assignment { 
            identifier: "yip".to_string(),
            value: Value::Int(5),
         })
    ];

    assert_eq!(statements, expected_statements);
}
}