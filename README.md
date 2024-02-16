# json parser

## Usage

```rust
let json_str = r#"
    {
      "name": "mitsuaki",
      "age": 24,
      "hobby": [
        "music",
        "movie"
      ]
    }
"#;

    let v = json::from_str(json_str).unwrap();

    println!("{:?}", v);
    // => Object({"hobby": Array([String("music"), String("movie")]), "name": String("mitsuaki"), "age": Number(24)})
```

## Development

### How to run test?

```shell
cargo test
```
