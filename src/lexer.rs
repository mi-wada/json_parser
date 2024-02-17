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
            c if c.is_ascii_digit() || c == '-' => {
                let mut number = String::new();
                number.push(c);
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        number.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if number.contains('.') {
                    tokens.push(Token::Number(Number::Float(
                        number.parse().expect("Invalid number"),
                    )));
                } else {
                    tokens.push(Token::Number(Number::Integer(
                        number.parse().expect("Invalid number"),
                    )));
                }
            }
            ' ' | '\n' | '\r' => continue,
            _ => bail!("Invalid input"),
        }
    }

    Ok(tokens)
}

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    String(String),
    Number(Number),
    True,
    False,
    Null,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Number {
    Integer(i64),
    Float(f64),
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

        assert_eq!(
            tokenize("123").ok().unwrap(),
            vec![Token::Number(Number::Integer(123))]
        );
        assert_eq!(
            tokenize("123.456").ok().unwrap(),
            vec![Token::Number(Number::Float(123.456))]
        );

        assert_eq!(tokenize("true").ok().unwrap(), vec![Token::True]);
        assert_eq!(tokenize("false").ok().unwrap(), vec![Token::False]);

        assert_eq!(tokenize("null").ok().unwrap(), vec![Token::Null]);

        assert!(tokenize("invalid").is_err());
    }
}
