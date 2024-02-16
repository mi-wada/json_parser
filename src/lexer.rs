use anyhow::{bail, Result};

pub(crate) fn tokenize(s: &str) -> Result<Vec<Token>> {
    match s {
        "true" => Ok(vec![Token::True]),
        "false" => Ok(vec![Token::False]),
        "null" => Ok(vec![Token::Null]),
        _ => bail!("Invalid input"),
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    True,
    False,
    Null,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("true").ok().unwrap(), vec![Token::True]);
        assert_eq!(tokenize("false").ok().unwrap(), vec![Token::False]);
        assert_eq!(tokenize("null").ok().unwrap(), vec![Token::Null]);

        assert!(tokenize("invalid").is_err());
    }
}
