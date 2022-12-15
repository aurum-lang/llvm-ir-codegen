#![allow(unused, dead_code)]
use crate::lexer::token_types::{Literal, Token};

// Unfinished helper type
// pub struct TokenStream {
//     list: Vec<Token>,
// }
//
// impl TokenStream {
//     pub fn iter(&self) -> TokenStreamIter {
//         TokenStreamIter {
//             internal: &self,
//             pos: 0,
//         }
//     }
// }
//
// pub struct TokenStreamIter<'a> {
//     internal: &'a TokenStream,
//     pos: usize,
// }
//
// impl<'a> Iterator for TokenStreamIter<'a> {
//     type Item = &'a Token;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let max: usize = self.internal.list.len();
//         if self.pos < max {
//             Some(&(self.internal.list)[max])
//         } else {
//             None
//         }
//     }
// }
//
// impl IntoIterator for &TokenStream {
//     fn into_iter(self) -> Self::IntoIter {
//     }
// }

// For our purposes a type alias provides all the methods we would implement anyways
pub type TokenStream = Vec<Token>;

fn parse_word(input: &str) -> Option<Token> {
    let s: String = input.to_string();
    let t: &str = s.trim();
    let p_res = s.parse::<i32>();
    let chars: Vec<char> = s.chars().collect();

    // TODO: Implement float support
    if chars.is_empty() {
        // Empty literal fails to tokenize
        None
    } else if *chars.first().unwrap() == '"' && *chars.last().unwrap() == '"' {
        // If literal is is wrapped in quotes, make it a string literal
        Some(Token::Literal(Literal::String(
            chars[1..(chars.len() - 1)].into_iter().collect(),
        )))
    } else if p_res.is_ok() {
        // If literal can be parsed as a i32, it's an int literal
        Some(Token::Literal(Literal::Int(p_res.unwrap())))
    } else if t == "true" || t == "false" {
        // if literal is either true or false it's used to represent a boolean state
        match t {
            "true" => Some(Token::Literal(Literal::Bool(true))),
            "false" => Some(Token::Literal(Literal::Bool(false))),
            _ => unreachable!(),
        }
    } else {
        None
    }
}

fn split_with_eq<'a>(input: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = vec![];
    let mut pos: usize = 0;
    for (i, c) in input.chars().enumerate() {
        if c == ' ' || c == '=' {
            result.push(&input[pos..i]);
            pos = i + 1;
        }
    }
    result
}

fn match_tokens(input: &str) -> TokenStream {
    let mut result: TokenStream = vec![];
    let input: String = input.to_string();
    let splits: Vec<&str> = split_with_eq(&input);
    for s in splits {
        if let Some(t) = parse_word(s) {
            result.push(t);
        }
    }
    result
}

pub fn get_token_stream(input: &str) -> TokenStream {
    let mut result: TokenStream = vec![];
    let input: String = input.to_string();

    for line in input.lines() {
        // DESIGN DECISION: Comments can only be on their own line
        // This reduces the compiler processing time per line
        if line.trim() == "" || line.starts_with("//") {
            continue;
        }

        let token_stream: TokenStream = match_tokens(line);
        for token in token_stream {
            result.push(token);
        }
    }
    result
}

pub fn tokenize<T: ToString>(input: T) -> Vec<u8> {
    let s: String = input.to_string();
    let token_stream: TokenStream = get_token_stream(&s);
    let mut buffer: String = String::new();

    for token in token_stream {
        match token {
            Token::Keyword(kword) => {
                buffer += stringify!("<Keyword({kword:?})>");
            }
            Token::Symbol(sym) => {
                buffer += stringify!("<Symbol({sym:?})>");
            }
            Token::Literal(lit) => {
                buffer += stringify!("<Literal({kword:?})>");
            }
            Token::Array(prim) => {
                buffer += stringify!("<Keyword({kword:?})>");
            }
            Token::Unknown(name) => {
                buffer += stringify!("<Keyword({kword:?})>");
            }
        }
    }

    buffer.into_bytes()
}
