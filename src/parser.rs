// AST

use core::panic;

use crate::lexer::Token;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AstNodeType {
    NumberLiteral,
    StringLiteral,
    CallExpression,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AstNode {
    pub r#type: AstNodeType,
    pub value: String,
    pub params: Option<Vec<AstNode>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Ast {
    pub body: Vec<AstNode>,
}

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Ast {
        let mut body: Vec<AstNode> = Vec::new();

        while self.current < self.tokens.len() {
            body.push(self.walk());
        }

        Ast { body }
    }

    pub fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn next(&mut self) {
        self.current += 1;
    }

    pub(self) fn walk(&mut self) -> AstNode {
        match self.current_token() {
            Token::Paren(p) if p.eq(&'(') => {
                self.next(); // skip parentesis

                // Get the name
                let mut name = String::new();
                if let Token::Name(v) = self.current_token() {
                    name = v.to_string();
                };
                self.next(); // skip name token

                let mut params: Vec<AstNode> = Vec::new();
                while !matches!(self.current_token(), Token::Paren(')')) {
                    params.push(self.walk());
                }
                self.next(); // skip the last closing parentesis

                AstNode {
                    r#type: AstNodeType::CallExpression,
                    value: name,
                    params: Some(params),
                }
            }
            Token::Text(v) => {
                let text_node = AstNode {
                    r#type: AstNodeType::StringLiteral,
                    value: v.to_owned(),
                    params: None,
                };

                self.next();

                text_node
            }
            Token::Number(v) => {
                let number_node = AstNode {
                    r#type: AstNodeType::NumberLiteral,
                    value: v.to_owned(),
                    params: None,
                };

                self.next();

                number_node
            }
            _ => panic!("Invalid token {:?}", self.current_token()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::{
        lexer::Tokenizer,
        parser::{Ast, AstNode, AstNodeType},
    };

    use super::Parser;

    #[test]
    fn should_turn_tokens_into_ast() {
        let tokens = Tokenizer::new(r#"(add 2 (subtract 4 6))"#).get_tokens();

        assert_eq!(
            Parser::new(tokens).parse(),
            Ast {
                body: vec![AstNode {
                    r#type: AstNodeType::CallExpression,
                    value: "add".to_string(),
                    params: Some(vec![
                        AstNode {
                            r#type: AstNodeType::NumberLiteral,
                            value: "2".to_string(),
                            params: None
                        },
                        AstNode {
                            r#type: AstNodeType::CallExpression,
                            value: "subtract".to_string(),
                            params: Some(vec![
                                AstNode {
                                    r#type: AstNodeType::NumberLiteral,
                                    value: "4".to_string(),
                                    params: None
                                },
                                AstNode {
                                    r#type: AstNodeType::NumberLiteral,
                                    value: "6".to_string(),
                                    params: None
                                }
                            ])
                        }
                    ])
                }]
            }
        )
    }

    #[test]
    fn should_turn_multiline_into_multi_ast_bodies() {
        let tokens = Tokenizer::new(
            r#"
            (add 2 (subtract 4 6))
            (concat "eduardo" "stuart")
        "#,
        )
        .get_tokens();

        let first = AstNode {
            r#type: AstNodeType::CallExpression,
            value: "add".to_string(),
            params: Some(vec![
                AstNode {
                    r#type: AstNodeType::NumberLiteral,
                    value: "2".to_string(),
                    params: None,
                },
                AstNode {
                    r#type: AstNodeType::CallExpression,
                    value: "subtract".to_string(),
                    params: Some(vec![
                        AstNode {
                            r#type: AstNodeType::NumberLiteral,
                            value: "4".to_string(),
                            params: None,
                        },
                        AstNode {
                            r#type: AstNodeType::NumberLiteral,
                            value: "6".to_string(),
                            params: None,
                        },
                    ]),
                },
            ]),
        };

        let second = AstNode {
            r#type: AstNodeType::CallExpression,
            value: "concat".to_string(),
            params: Some(vec![
                AstNode {
                    r#type: AstNodeType::StringLiteral,
                    value: "eduardo".to_string(),
                    params: None,
                },
                AstNode {
                    r#type: AstNodeType::StringLiteral,
                    value: "stuart".to_string(),
                    params: None,
                },
            ]),
        };

        assert_eq!(
            Parser::new(tokens).parse(),
            Ast {
                body: vec![first, second]
            }
        )
    }
}
