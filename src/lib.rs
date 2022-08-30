mod lexer;
mod parser;

use anyhow::Result;
use parser::Parser;

use crate::parser::Value;

pub fn from_str(str: &str) -> Result<Value> {
    Parser::new(str)?.parse()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_null() {
        let json_str = "null";
        assert_eq!(from_str(json_str).unwrap(), Value::Null);
    }

    #[test]
    fn test_boolean() {
        let json_str = "true";
        assert_eq!(from_str(json_str).unwrap(), Value::Boolean(true));

        let json_str = "false";
        assert_eq!(from_str(json_str).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn test_number() {
        let json_str = "1";
        assert_eq!(from_str(json_str).unwrap(), Value::Number(1));
    }

    #[test]
    fn test_string() {
        let json_str = r#""str""#;
        assert_eq!(
            from_str(json_str).unwrap(),
            Value::String("str".to_string())
        );
    }

    #[test]
    fn test_array() {
        let json_str = r#"
        [
            "hoge",
            1234,
            true,
            null
        ]
        "#;

        assert_eq!(
            from_str(json_str).unwrap(),
            Value::Array(vec![
                Value::String(String::from("hoge")),
                Value::Number(1234),
                Value::Boolean(true),
                Value::Null
            ])
        );
    }

    #[test]
    fn test_empty_array() {
        let json_str = "[]";

        assert_eq!(from_str(json_str).unwrap(), Value::Array(vec![]))
    }

    #[test]
    fn test_object() {
        let json_str = r#"
        {
            "a": null,
            "b": true,
            "c": 1,
            "d": "str",
            "e": [
                null
            ]
        }
        "#;

        let expected_object = {
            let mut expected_object = HashMap::new();

            expected_object.insert("a".to_string(), Value::Null);
            expected_object.insert("b".to_string(), Value::Boolean(true));
            expected_object.insert("c".to_string(), Value::Number(1));
            expected_object.insert("d".to_string(), Value::String("str".to_string()));
            expected_object.insert("e".to_string(), Value::Array(vec![Value::Null]));

            expected_object
        };
        assert_eq!(from_str(json_str).unwrap(), Value::Object(expected_object));
    }

    #[test]
    fn test_empty_object() {
        let json_str = "{}";

        assert_eq!(from_str(json_str).unwrap(), Value::Object(HashMap::new()));
    }
}
