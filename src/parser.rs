use std::{collections::HashMap, iter::Peekable};

use anyhow::{bail, Ok, Result};

use crate::lexer::{self, Token};

#[derive(Debug, PartialEq)]
pub enum Value {
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
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

pub(crate) fn parse(tokens: Vec<Token>) -> Result<Value> {
    json(&mut tokens.into_iter().peekable())
}

fn json<I>(tokens: &mut Peekable<I>) -> Result<Value>
where
    I: Iterator<Item = Token>,
{
    match tokens.peek() {
        Some(token) => match token {
            Token::String(ref s) => string(tokens),
            Token::Number(lexer::Number::Integer(ref n)) => {
                let n = n.clone();
                tokens.next();
                Ok(Value::Number(Number::Integer(n)))
            }
            Token::Number(lexer::Number::Float(ref n)) => {
                let n = n.clone();
                tokens.next();
                Ok(Value::Number(Number::Float(n)))
            }
            Token::True => {
                tokens.next();
                Ok(Value::Bool(true))
            }
            Token::False => {
                tokens.next();
                Ok(Value::Bool(false))
            }
            Token::Null => {
                tokens.next();
                Ok(Value::Null)
            }
            Token::LBracket => array(tokens),
            Token::LBrace => object(tokens),
            Token::RBracket | Token::RBrace | Token::Comma | Token::Colon => bail!("Invalid input"),
        },
        None => bail!("Invalid input"),
    }
}

fn array<I>(tokens: &mut Peekable<I>) -> Result<Value>
where
    I: Iterator<Item = Token>,
{
    tokens.next().unwrap();

    let mut values = vec![];
    loop {
        match tokens.peek() {
            Some(Token::RBracket) => {
                tokens.next();
                break;
            }
            Some(Token::Comma) => {
                tokens.next();
                continue;
            }
            _ => {
                values.push(json(tokens)?);
            }
        }
    }

    Ok(Value::Array(values))
}

fn object<I>(tokens: &mut Peekable<I>) -> Result<Value>
where
    I: Iterator<Item = Token>,
{
    consume(tokens, Token::LBrace)?;

    let mut hash_map = HashMap::new();

    loop {
        let key = string(tokens)?;
        consume(tokens, Token::Colon)?;
        let value = json(tokens)?;

        match key {
            Value::String(key) => {
                hash_map.insert(key, value);
            }
            _ => {
                panic!("Bug. Not string key passed.");
            }
        }

        match tokens.next() {
            Some(Token::Comma) => continue,
            Some(Token::RBrace) => {
                break;
            }
            Some(_) | None => {
                bail!("invalid input")
            }
        }
    }

    Ok(Value::Object(hash_map))
}

fn string<I>(tokens: &mut Peekable<I>) -> Result<Value>
where
    I: Iterator<Item = Token>,
{
    match tokens.next() {
        Some(Token::String(s)) => Ok(Value::String(s)),
        _ => bail!("invalid input"),
    }
}

fn consume<I>(tokens: &mut Peekable<I>, expected_token: Token) -> Result<Token>
where
    I: Iterator<Item = Token>,
{
    match tokens.next() {
        Some(next_token) => {
            if next_token == expected_token {
                Ok(next_token)
            } else {
                bail!("Invalid input")
            }
        }
        None => bail!("Invalid input"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!(
            parse(tokenize(r#"{"hello": [12, "abc"], "world": null}"#)?)
                .ok()
                .unwrap(),
            Value::Object(
                vec![
                    (
                        "hello".to_string(),
                        Value::Array(vec![
                            Value::Number(Number::Integer(12)),
                            Value::String("abc".to_string())
                        ])
                    ),
                    ("world".to_string(), Value::Null)
                ]
                .into_iter()
                .collect()
            )
        );

        assert_eq!(
            parse(tokenize(r#"["hello", 123, ["hello", 123]]"#).unwrap())
                .ok()
                .unwrap(),
            Value::Array(vec![
                Value::String("hello".to_string()),
                Value::Number(Number::Integer(123)),
                Value::Array(vec![
                    Value::String("hello".to_string()),
                    Value::Number(Number::Integer(123))
                ])
            ])
        );

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

        Ok(())
    }
}
