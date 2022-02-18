use compiler_lib::Tokenizer;

fn main() {
    let input = r#"(add 2 (subtract 3 5))"#;
    let tokens = Tokenizer::new(input).get_tokens();

    println!(
        "
Input: 
{}
Tokens: 
{:?}
    ",
        input, tokens
    );
}
