use compiler_lib::compile;

fn main() {
    let input = r#"(add 2 (subtract 3 5))"#;
    let output = compile(input);

    println!(
        "
Input: 
{}
Compiled:
{}
    ",
        input, output
    );
}
