use anyhow::Result;

use crate::lexer::Token;

pub(crate) fn parse(tokens: Vec<Token>) -> Result<Value> {
    match tokens[0] {
        Token::True => Ok(Value::Bool(true)),
        Token::False => Ok(Value::Bool(false)),
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Bool(bool),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(tokenize("true").unwrap()).ok().unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            parse(tokenize("false").unwrap()).ok().unwrap(),
            Value::Bool(false)
        );
    }
}
