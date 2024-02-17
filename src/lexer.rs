use anyhow::{bail, Result};

pub(crate) fn tokenize(s: &str) -> Result<Vec<Token>> {
    let mut tokens = vec![];

    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            't' => {
                if chars.next() == Some('r')
                    && chars.next() == Some('u')
                    && chars.next() == Some('e')
                {
                    tokens.push(Token::True);
                } else {
                    bail!("Invalid input");
                }
            }
            'f' => {
                if chars.next() == Some('a')
                    && chars.next() == Some('l')
                    && chars.next() == Some('s')
                    && chars.next() == Some('e')
                {
                    tokens.push(Token::False);
                } else {
                    bail!("Invalid input");
                }
            }
            'n' => {
                if chars.next() == Some('u')
                    && chars.next() == Some('l')
                    && chars.next() == Some('l')
                {
                    tokens.push(Token::Null);
                } else {
                    bail!("Invalid input");
                }
            }
            '"' => {
                let mut string = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' {
                        tokens.push(Token::String(string));
                        break;
                    } else {
                        string.push(c);
                    }
                }
            }
            ' ' | '\n' | '\r' => continue,
            _ => bail!("Invalid input"),
        }
    }
    return Ok(tokens);
}

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    String(String),
    True,
    False,
    Null,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize(r#""hello""#).ok().unwrap(),
            vec![Token::String("hello".to_string())]
        );

        assert_eq!(tokenize("true").ok().unwrap(), vec![Token::True]);
        assert_eq!(tokenize("false").ok().unwrap(), vec![Token::False]);

        assert_eq!(tokenize("null").ok().unwrap(), vec![Token::Null]);

        assert!(tokenize("invalid").is_err());
    }
}
