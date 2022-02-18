use std::{iter::Peekable, str::CharIndices};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Paren(char),
    Text(String),
    Number(String),
    Name(String),
}

pub struct Tokenizer<'a> {
    iter: Peekable<CharIndices<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            iter: input.char_indices().peekable(),
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some((_, ch)) = self.iter.peek() {
            let c = *ch;

            match c {
                '(' | ')' => {
                    tokens.push(Token::Paren(c));
                    self.iter.next();
                }
                c if c.is_whitespace() => {
                    self.consume_whitespace();
                }
                c if c.is_numeric() => {
                    tokens.push(Token::Number(self.consume_number()));
                }
                '"' => {
                    self.iter.next(); // skip "
                    tokens.push(Token::Text(self.consume_text()));
                    self.iter.next(); // skip "
                }
                _ => {
                    tokens.push(Token::Name(self.consume_name()));
                }
            };
        }

        tokens
    }

    pub(self) fn consume_whitespace(&mut self) {
        while let Some((_, c)) = self.iter.peek() {
            if c.is_whitespace() {
                self.iter.next();
            } else {
                break;
            }
        }
    }

    // Get the token type number
    // Iterate until there are no more numbers
    pub(self) fn consume_number(&mut self) -> String {
        let mut text = String::new();
        while let Some((_, c)) = self.iter.peek() {
            if c.is_whitespace() {
                self.iter.next();
                break;
            } else if !c.is_numeric() && !matches!(c, '.' | ',') {
                break;
            }
            text.push(*c);
            self.iter.next();
        }
        text
    }

    pub(self) fn consume_name(&mut self) -> String {
        let mut name = String::new();
        while let Some((_, c)) = self.iter.peek() {
            if !c.is_alphanumeric() {
                self.iter.next();
                break;
            }

            name.push(*c);
            self.iter.next();
        }
        name
    }

    // Iterate and get text values
    pub(self) fn consume_text(&mut self) -> String {
        let mut text = String::new();
        while let Some((_, c)) = self.iter.peek() {
            if c.is_whitespace() {
                self.iter.next();
                break;
            } else if matches!(c, '"' | '\'') {
                break;
            }

            text.push(*c);
            self.iter.next();
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::{Token, Tokenizer};

    #[test]
    fn should_get_list_of_tokens_from_string() {
        assert_eq!(
            Tokenizer::new(r#"(add 2)"#).get_tokens(),
            vec![
                Token::Paren('('),
                Token::Name("add".to_string()),
                Token::Number("2".to_string()),
                Token::Paren(')'),
            ]
        );

        assert_eq!(
            Tokenizer::new(r#"(add 2 (subtract 4 6))"#).get_tokens(),
            vec![
                Token::Paren('('),
                Token::Name("add".to_string()),
                Token::Number("2".to_string()),
                Token::Paren('('),
                Token::Name("subtract".to_string()),
                Token::Number("4".to_string()),
                Token::Number("6".to_string()),
                Token::Paren(')'),
                Token::Paren(')'),
            ]
        );
    }
}
