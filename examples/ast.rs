use compiler_lib::{Parser, Tokenizer};

fn main() {
    let input = r#"(add 2 (subtract 3 5))"#;
    let tokens = Tokenizer::new(input).get_tokens();
    let ast = Parser::new(tokens).parse();

    println!(
        "
Input: 
{}
Ast: 
{:?}
    ",
        input, ast
    );
}
