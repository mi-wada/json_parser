use std::collections::HashMap;

use anyhow::{anyhow, Ok, Result};

use crate::lexer::{Lexer, Token};

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Null,
    Boolean(bool),
    // TODO: impl Number struct
    Number(u64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

pub struct Parser {
    tokens: std::iter::Peekable<std::vec::IntoIter<Token>>,
}

impl Parser {
    pub fn new(json_str: &str) -> Result<Self> {
        let tokens = Lexer::new(json_str).tokenize()?;

        Ok(Parser {
            tokens: tokens.into_iter().peekable(),
        })
    }

    pub fn parse(&mut self) -> Result<Value> {
        if let Some(token) = self.tokens.next() {
            match token {
                Token::Null => Ok(Value::Null),
                Token::Bool(b) => Ok(Value::Boolean(b)),
                Token::Number(n) => Ok(Value::Number(n)),
                Token::String(s) => Ok(Value::String(s)),
                Token::LeftBracket => self.parse_array(),
                Token::LeftBrace => self.parse_object(),
                _ => Err(anyhow!("got unexpected token: {:?}", token)),
            }
        } else {
            Err(anyhow!("EOF error"))
        }
    }

    fn parse_array(&mut self) -> Result<Value> {
        let mut array = vec![];

        loop {
            if let Some(token) = self.tokens.peek() && token == &Token::RightBracket {
                self.tokens.next();
                break;
            }

            array.push(self.parse()?);

            if let Some(token) = self.tokens.next() {
                match token {
                    Token::RightBracket => {
                        break;
                    }
                    Token::Comma => {
                        continue;
                    }
                    _ => return Err(anyhow!("found unexpected token: {:?}", token)),
                }
            } else {
                return Err(anyhow!("expected , or ], but not found"));
            }
        }

        Ok(Value::Array(array))
    }

    fn parse_object(&mut self) -> Result<Value> {
        let mut object = HashMap::<String, Value>::new();

        while let Some(token) = self.tokens.next() {
            let key = {
                match token {
                    Token::String(s) => s,
                    Token::RightBrace => {
                        break;
                    }
                    _ => {
                        return Err(anyhow!(
                            "failed to parse Object. unexpected token: {:?}",
                            token
                        ))
                    }
                }
            };

            if let Some(probably_colon) = self.tokens.next() {
                if probably_colon != Token::Colon {
                    return Err(anyhow!(
                        "expected to exist colon but exist {:?}",
                        probably_colon
                    ));
                }
            } else {
                return Err(anyhow!("expected to exist colon but not exist"));
            }

            let value = self.parse()?;

            object.insert(key, value);

            match self.tokens.next() {
                Some(Token::Comma) => {
                    continue;
                }
                Some(Token::RightBrace) => {
                    break;
                }
                Some(token) => return Err(anyhow!("unexpected token: {:?}", token)),
                None => return Err(anyhow!("object must be end with , or RightBackrace")),
            }
        }

        Ok(Value::Object(object))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        let json_str = "null";
        assert_eq!(helper::parse(json_str).unwrap(), Value::Null);
    }

    #[test]
    fn test_bool() {
        let json_str = "true";
        assert_eq!(helper::parse(json_str).unwrap(), Value::Boolean(true));

        let json_str = "false";
        assert_eq!(helper::parse(json_str).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn test_number() {
        let json_str = "1";
        assert_eq!(helper::parse(json_str).unwrap(), Value::Number(1));
    }

    #[test]
    fn test_string() {
        let json_str = r#""str""#;
        assert_eq!(
            helper::parse(json_str).unwrap(),
            Value::String("str".to_string())
        );
    }

    #[test]
    fn test_array() {
        let json_str = r#"
        [
            "hoge",
            1234,
            true,
            null
        ]
        "#;

        assert_eq!(
            helper::parse(json_str).unwrap(),
            Value::Array(vec![
                Value::String(String::from("hoge")),
                Value::Number(1234),
                Value::Boolean(true),
                Value::Null
            ])
        );
    }

    #[test]
    fn test_object() {
        let json_str = r#"
        {
            "a": null,
            "b": true,
            "c": 1,
            "d": "str",
            "e": [
                null
            ]
        }
        "#;

        let expected_object = {
            let mut expected_object = HashMap::new();

            expected_object.insert("a".to_string(), Value::Null);
            expected_object.insert("b".to_string(), Value::Boolean(true));
            expected_object.insert("c".to_string(), Value::Number(1));
            expected_object.insert("d".to_string(), Value::String("str".to_string()));
            expected_object.insert("e".to_string(), Value::Array(vec![Value::Null]));

            expected_object
        };

        assert_eq!(
            helper::parse(json_str).unwrap(),
            Value::Object(expected_object)
        );
    }

    mod helper {
        use super::*;

        pub fn parse(str: &str) -> Result<Value> {
            Parser::new(str)?.parse()
        }
    }
}
