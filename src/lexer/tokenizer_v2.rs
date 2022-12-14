#![allow(unused, dead_code)]
use crate::lexer::token_types::Token;

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

pub type TokenStream = Vec<Token>;

fn match_word<T: ToString>(input: T) -> Option<Token> {
    let s: String = input.to_string();
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        None
    } else if *chars.first().unwrap() == '"' && *chars.last().unwrap() == '"' {
        Some(Token::Literal(s))
    } else {
        None
    }
}

fn match_token<T: ToString>(input: T) -> TokenStream {
    let mut buffer1: String = String::new();
    let mut buffer2: String = String::new();
    todo!()
}

pub fn tokenize<T: ToString>(input: T) -> TokenStream {
    todo!()
}
