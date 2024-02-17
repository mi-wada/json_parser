mod lexer;
mod parser;

use anyhow::Result;
use parser::Value;

pub fn from_str(s: &str) -> Result<Value> {
    parser::parse(lexer::tokenize(s)?)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            from_str(r#"["hello", 123, ["hello", 123]]"#).ok().unwrap(),
            Value::Array(vec![
                Value::String("hello".to_string()),
                Value::Number(parser::Number::Integer(123)),
                Value::Array(vec![
                    Value::String("hello".to_string()),
                    Value::Number(parser::Number::Integer(123))
                ])
            ])
        );

        assert_eq!(
            from_str(r#""hello""#).ok().unwrap(),
            Value::String("hello".to_string())
        );

        assert_eq!(
            from_str("123").ok().unwrap(),
            Value::Number(parser::Number::Integer(123))
        );
        assert_eq!(
            from_str("123.456").ok().unwrap(),
            Value::Number(parser::Number::Float(123.456))
        );
        assert_eq!(
            from_str("-123.456").ok().unwrap(),
            Value::Number(parser::Number::Float(-123.456))
        );

        assert_eq!(from_str("true").ok().unwrap(), Value::Bool(true));
        assert_eq!(from_str("false").ok().unwrap(), Value::Bool(false));

        assert_eq!(from_str("null").ok().unwrap(), Value::Null);

        assert!(from_str("invalid").is_err());
    }
}
