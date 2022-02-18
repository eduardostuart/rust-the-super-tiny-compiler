mod compiler;
mod lexer;
mod parser;

pub use compiler::Compiler;
pub use lexer::{Token, Tokenizer};
pub use parser::{Ast, AstNode, AstNodeType, Parser};

pub fn compile(input: &str) -> String {
    let tokens = Tokenizer::new(input).get_tokens();
    let ast = Parser::new(tokens).parse();
    Compiler::new(ast).compile()
}

#[cfg(test)]
mod test {
    use crate::compile;

    #[test]
    fn should_compile_from_str() {
        assert_eq!(compile("(add 2 (subtract 4 6))"), "add(2, subtract(4, 6));");
        assert_eq!(
            compile("(concat \"hello\" \"world\")"),
            "concat(\"hello\", \"world\");"
        );
    }
}
