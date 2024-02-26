use language::lexer::{Lexer, Token};

fn main() {
    let input = r#"
    var a;
    a = 1 + 2;
    function add(a, b) {
        return a + b;
    }
    "#;

    let mut lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.by_ref().collect();

    for token in tokens {
        println!("{:?}", token);
    }
}
