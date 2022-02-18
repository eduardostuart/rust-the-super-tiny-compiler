use crate::parser::{Ast, AstNode, AstNodeType};

#[derive(Debug)]
pub struct Compiler {
    ast: Ast,
}

impl Compiler {
    pub fn new(ast: Ast) -> Self {
        Self { ast }
    }

    pub fn compile(&mut self) -> String {
        let mut out: Vec<String> = Vec::new();
        let body = self.ast.body.iter().peekable();

        for p in body {
            out.push(format!("{};", self.generate(p)));
        }

        out.join("\n")
    }

    pub(self) fn generate(&self, node: &AstNode) -> String {
        match node.r#type {
            AstNodeType::NumberLiteral => node.value.to_string(),
            AstNodeType::StringLiteral => format!("\"{}\"", node.value),
            AstNodeType::CallExpression => {
                let params = node.params.as_ref().unwrap().to_vec();

                let exp = params
                    .iter()
                    .map(|n| self.generate(n))
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("{}({})", node.value, exp)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::Tokenizer, parser};

    use super::Compiler;

    fn get_compiled_code_from(input: &str) -> String {
        let tokens = Tokenizer::new(input).get_tokens();
        let ast = parser::Parser::new(tokens).parse();
        let code = Compiler::new(ast).compile();
        code
    }

    #[test]
    pub fn should_compile_ast_int_string() {
        assert_eq!(
            get_compiled_code_from(r#"(add 2 (subtract 4 6))"#),
            "add(2, subtract(4, 6));"
        );
        assert_eq!(
            get_compiled_code_from(
                r#"
                (add 4 (subtract 2 3 (add 4)))
                (concat "E" "duardo")
            "#
            ),
            "add(4, subtract(2, 3, add(4)));\nconcat(\"E\", \"duardo\");"
        );
        assert_eq!(get_compiled_code_from(r#"(add 2)"#), "add(2);");
    }
}
