use anyhow::bail;
use anyhow::Result;

pub fn from_str(s: &str) -> Result<Value> {
    match s {
        "true" => Ok(Value::Bool(true)),
        "false" => Ok(Value::Bool(false)),
        _ => bail!("Invalid input"),
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Bool(bool),
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
