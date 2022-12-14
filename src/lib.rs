mod lexer;

#[cfg(test)]
mod tests {
    use crate::lexer;

    #[test]
    pub fn lexer_test() {
        let content: String = std::fs::read_to_string("./test.au").unwrap();
        let _ = std::fs::write("test.lex", lexer::tokenize(content).unwrap());
    }
}
