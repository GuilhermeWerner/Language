use language::lexer::{Lexer, TokenKind};

fn main() {
    let input = r#"
    # This is a comment

    var var1 = 1;
    var1 = var1 + 1;

    function add(a, b) {
        return a + b;
    }

    const var2 = 2;
    const var3 = add(var1, var2);
    "#;

    let mut lexer = Lexer::new(input);
    let tokens: Vec<TokenKind> = lexer.by_ref().collect();

    for token in tokens {
        println!("{:?}", token);
    }
}
