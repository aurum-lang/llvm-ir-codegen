#![allow(unused, dead_code)]
mod lexer;

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::lexer;

    #[test]
    pub fn lexer_test() {
        let content: String = std::fs::read_to_string("./test.au").unwrap();
        let _ = std::fs::write("test.lex", lexer::tokenize(content));
    }

    #[test]
    pub fn logos_test() {
        let content: String = std::fs::read_to_string("./test.au").unwrap();
        let mut lex = lexer::Token::lexer(&content);

        loop {
            match lex.next() {
                Some(token) => {
                    println!("{token:?} | {}", lex.slice());
                }
                None => break,
            }
        }
        // println!("{token:?} | {}", lex.slice());
    }
}
