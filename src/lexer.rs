use anyhow::{bail, Result};

pub(crate) fn tokenize(s: &str) -> Result<Vec<Token>> {
    match s {
        "true" => Ok(vec![Token::True]),
        "false" => Ok(vec![Token::False]),
        _ => bail!("Invalid input"),
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    True,
    False,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("true").ok().unwrap(), vec![Token::True]);
        assert_eq!(tokenize("false").ok().unwrap(), vec![Token::False]);

        assert!(tokenize("invalid").is_err());
    }
}
