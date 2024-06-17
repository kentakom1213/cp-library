/// DFSにより，再帰的に依存関係を列挙する
pub fn enum_dependancies_recursive() {}

/// ソースコードをトークン列に分解する
fn tokenize(source: &str) -> Vec<String> {
    let mut tokens = vec![];

    for t in source.split_ascii_whitespace() {
        let mut last = 0;
        for (i, c) in t.chars().enumerate() {
            match c {
                ',' | ';' | '{' | '}' | '(' | ')' => {
                    tokens.push(t[last..i].to_string());
                    tokens.push(t[i..=i].to_string());
                    last = i + 1;
                }
                ':' => {
                    if i + 1 < t.len() && &t[i..=i + 1] == "::" {
                        tokens.push(t[last..i].to_string());
                        tokens.push(t[i..=i + 1].to_string());
                        last = i + 2;
                    } else if last <= i {
                        tokens.push(t[last..i].to_string());
                        tokens.push(t[i..=i].to_string());
                        last = i + 1;
                    }
                }
                _ => (),
            }
        }
        if last < t.len() {
            tokens.push(t[last..].to_string());
        }
    }

    tokens
}

#[cfg(test)]
mod test_utils {
    use crate::utils::tokenize;

    #[test]
    fn test_parse_dependancies() {
        let source1 = r#"use crate::{a, b::{c, d}, e};
use std::collections::BTreeMap;

fn func<T>(t: T)
where
    T: Ord,
{
    todo!()
}

fn main() {
    println!("Hello, World");
    return 0;
}"#;

        println!("{}", source1);

        let tokens = tokenize(source1);

        println!("{:?}", tokens);
    }
}
