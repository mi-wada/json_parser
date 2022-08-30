use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    String(String), // 文字列
    Number(u64),    // 数値
    Bool(bool),     // 真偽値
    Null,           // Null
    LeftBrace,      // {　JSON object 開始文字
    RightBrace,     // }　JSON object 終了文字
    LeftBracket,    // [　JSON array  開始文字
    RightBracket,   // ]　JSON array  終了文字
    Comma,          // ,　JSON value  区切り文字
    Colon,          // :　"key":value 区切り文字
}

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(json_str: &str) -> Lexer {
        Lexer {
            chars: json_str.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = vec![];

        while let Some(c) = self.chars.peek() {
            match *c {
                '"' => {
                    tokens.push(self.try_to_read_string()?);
                }
                '0'..='9' => {
                    tokens.push(self.read_number());
                }
                't' | 'f' => {
                    tokens.push(self.try_to_read_bool()?);
                }
                'n' => {
                    tokens.push(self.try_to_read_null()?);
                }
                '{' => {
                    tokens.push(Token::LeftBrace);
                    self.chars.next();
                }
                '}' => {
                    tokens.push(Token::RightBrace);
                    self.chars.next();
                }
                '[' => {
                    tokens.push(Token::LeftBracket);
                    self.chars.next();
                }
                ']' => {
                    tokens.push(Token::RightBracket);
                    self.chars.next();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.chars.next();
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.chars.next();
                }
                // 空白、改行は無視しよう
                ' ' | '\n' => {
                    self.chars.next();
                }
                c => {
                    return Err(anyhow!("unexpected char: {}", c));
                }
            }
        }

        Ok(tokens)
    }

    fn try_to_read_null(&mut self) -> Result<Token> {
        if let Some(c1) = self.chars.next()
            && let Some(c2) = self.chars.next()
            && let Some(c3) = self.chars.next()
            && let Some(c4) = self.chars.next()
        {
            if c1 == 'n' && c2 == 'u' && c3 == 'l' && c4 == 'l' {
                Ok(Token::Null)
            } else {
                Err(anyhow!("expected `null` but get `{}{}{}{}`", c1, c2, c3, c4))
            }
        } else {
            Err(anyhow!("failed to parse null"))
        }
    }

    fn try_to_read_bool(&mut self) -> Result<Token> {
        if let Some(c1) = self.chars.next()
            && let Some(c2) = self.chars.next()
            && let Some(c3) = self.chars.next()
            && let Some(c4) = self.chars.next()
        {
            if c1 == 't' && c2 == 'r' && c3 == 'u' && c4 == 'e' {
                return Ok(Token::Bool(true));
            }

            if let Some(c5) = self.chars.next() {
                if c1 == 'f' && c2 == 'a' && c3 == 'l' && c4 == 's' && c5 == 'e' {
                    Ok(Token::Bool(false))
                } else {
                    Err(anyhow!("expected `false` but get `{}{}{}{}{}`", c1, c2, c3, c4, c5))
                }
            } else {
                Err(anyhow!("failed to parse boolean value"))
            }
        } else {
            Err(anyhow!("failed to parse boolean value"))
        }
    }

    fn read_number(&mut self) -> Token {
        let mut digit_chars = vec![];
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_digit() {
                digit_chars.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }

        Token::Number(String::from_iter(digit_chars).parse::<u64>().unwrap())
    }

    fn try_to_read_string(&mut self) -> Result<Token> {
        let mut chars = vec![];

        if let Some(maybe_double_quotation) = self.chars.next() {
            if maybe_double_quotation != '"' {
                return Err(anyhow!("maybe miss implementation"));
            }
        } else {
            return Err(anyhow!("maybe miss implementation"));
        }

        let mut end_with_double_quotation = false;

        while let Some(c) = self.chars.peek() {
            if *c == '"' {
                self.chars.next();
                end_with_double_quotation = true;
                break;
            }

            chars.push(*c);
            self.chars.next();
        }

        if end_with_double_quotation {
            Ok(Token::String(String::from_iter(chars)))
        } else {
            Err(anyhow!("missing end of double quotation"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        let mut lexer = Lexer::new("null");
        assert_eq!(lexer.tokenize().unwrap(), vec![Token::Null]);
    }

    #[test]
    fn test_bool() {
        let mut lexer = Lexer::new("true");
        assert_eq!(lexer.tokenize().unwrap(), vec![Token::Bool(true)]);

        let mut lexer = Lexer::new("false");
        assert_eq!(lexer.tokenize().unwrap(), vec![Token::Bool(false)]);
    }

    #[test]
    fn test_number() {
        let mut lexer = Lexer::new("1234");
        assert_eq!(lexer.tokenize().unwrap(), vec![Token::Number(1234)]);
    }

    #[test]
    fn test_string() {
        let mut lexer = Lexer::new(r#""string""#);
        assert_eq!(
            lexer.tokenize().unwrap(),
            vec![Token::String(String::from("string"))]
        );
    }

    #[test]
    fn test_array() {
        let mut lexer = Lexer::new(
            r#"
        [
            null,
            true,
            false,
            "string",
            1234
        ]
        "#,
        );
        assert_eq!(
            lexer.tokenize().unwrap(),
            vec![
                Token::LeftBracket,
                Token::Null,
                Token::Comma,
                Token::Bool(true),
                Token::Comma,
                Token::Bool(false),
                Token::Comma,
                Token::String(String::from("string")),
                Token::Comma,
                Token::Number(1234),
                Token::RightBracket
            ]
        );
    }

    #[test]
    fn test_object() {
        let mut lexer = Lexer::new(
            r#"
        {
            "name": "mitsuaki",
            "age": 24,
            "is_admin": true,
            "spouse": null,
            "hobby": [
                "music",
                "movie"
            ]
        }
        "#,
        );
        assert_eq!(
            lexer.tokenize().unwrap(),
            vec![
                Token::LeftBrace,
                Token::String(String::from("name")),
                Token::Colon,
                Token::String(String::from("mitsuaki")),
                Token::Comma,
                Token::String(String::from("age")),
                Token::Colon,
                Token::Number(24),
                Token::Comma,
                Token::String(String::from("is_admin")),
                Token::Colon,
                Token::Bool(true),
                Token::Comma,
                Token::String(String::from("spouse")),
                Token::Colon,
                Token::Null,
                Token::Comma,
                Token::String(String::from("hobby")),
                Token::Colon,
                Token::LeftBracket,
                Token::String(String::from("music")),
                Token::Comma,
                Token::String(String::from("movie")),
                Token::RightBracket,
                Token::RightBrace
            ]
        );
    }
}
