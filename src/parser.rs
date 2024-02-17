use anyhow::Result;

use crate::lexer::{self, Token};

pub(crate) fn parse(tokens: Vec<Token>) -> Result<Value> {
    match tokens[0] {
        Token::String(ref s) => Ok(Value::String(s.clone())),
        Token::Number(lexer::Number::Integer(n)) => Ok(Value::Number(Number::Integer(n))),
        Token::Number(lexer::Number::Float(n)) => Ok(Value::Number(Number::Float(n))),
        Token::True => Ok(Value::Bool(true)),
        Token::False => Ok(Value::Bool(false)),
        Token::Null => Ok(Value::Null),
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(Number),
    Bool(bool),
    Null,
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(tokenize(r#""hello""#).unwrap()).ok().unwrap(),
            Value::String("hello".to_string())
        );

        assert_eq!(
            parse(tokenize("123").unwrap()).ok().unwrap(),
            Value::Number(Number::Integer(123))
        );
        assert_eq!(
            parse(tokenize("123.456").unwrap()).ok().unwrap(),
            Value::Number(Number::Float(123.456))
        );

        assert_eq!(
            parse(tokenize("true").unwrap()).ok().unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            parse(tokenize("false").unwrap()).ok().unwrap(),
            Value::Bool(false)
        );

        assert_eq!(parse(tokenize("null").unwrap()).ok().unwrap(), Value::Null);
    }
}
