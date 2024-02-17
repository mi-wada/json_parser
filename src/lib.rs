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
            from_str(r#""hello""#).ok().unwrap(),
            Value::String("hello".to_string())
        );

        assert_eq!(from_str("true").ok().unwrap(), Value::Bool(true));
        assert_eq!(from_str("false").ok().unwrap(), Value::Bool(false));

        assert_eq!(from_str("null").ok().unwrap(), Value::Null);

        assert!(from_str("invalid").is_err());
    }
}
