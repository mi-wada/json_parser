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
    fn test_bool() {
        assert_eq!(from_str("true").ok().unwrap(), Value::Bool(true));
        assert_eq!(from_str("false").ok().unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_invalid() {
        assert!(from_str("invalid").is_err());
    }
}
