#[derive(Debug, PartialEq, Eq)]
pub enum Lexem {
    Url(String),
}
impl Lexem {}
pub struct Lexer {
    idx: usize,
    parsed_lexems: Vec<Lexem>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            idx: 0,
            parsed_lexems: vec![],
        }
    }
    pub fn lex(&mut self, contents: &str) {
        let chars: Vec<char> = contents.chars().collect();

        while self.idx < chars.len() {
            let mut ch = chars[self.idx];

            // skip whitespaces
            while ch.is_ascii_whitespace() {
                self.idx += 1;
                if self.idx >= chars.len() {
                    return;
                }
                ch = chars[self.idx];
            }

            match ch {
                // url start
                'h' => {
                    self.parse_url(&chars);
                }
                _ => {
                    self.idx += 1;
                }
            }
        }
    }
    fn parse_url(&mut self, chars: &[char]) {
        let mut url_lexem = String::new();
        for expected_char in "http".chars() {
            if expected_char != chars[self.idx] {
                return;
            }
            url_lexem.push(chars[self.idx]);
            self.idx += 1;
        }
        if chars[self.idx] == 's' {
            url_lexem.push(chars[self.idx]);
            self.idx += 1;
        }

        for expected_char in "://".chars() {
            if expected_char != chars[self.idx] {
                return;
            }
            url_lexem.push(chars[self.idx]);
            self.idx += 1;
        }
        while self.idx < chars.len() && !chars[self.idx].is_ascii_whitespace() {
            url_lexem.push(chars[self.idx]);
            self.idx += 1;
        }
        self.parsed_lexems.push(Lexem::Url(url_lexem));
    }

    pub fn print_lexems(&self) {
        for lexem in &self.parsed_lexems {
            match lexem {
                Lexem::Url(url) => {
                    println!("Url: {:?}", url);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;

    fn process(input: &str) -> Vec<Lexem> {
        let mut lexer = Lexer::new();
        lexer.lex(input);
        lexer.parsed_lexems
    }

    #[test]
    fn test_simple() {
        let input = "https://example.com";
        let lexems = process(input);
        match &lexems[0] {
            Lexem::Url(url) => assert_eq!(url, input),
            _ => panic!("Expected url lexem"),
        }
    }

    #[test]
    fn test_empty() {
        let input = "";
        let lexems = process(input);
        assert_eq!(lexems.first(), None::<&Lexem>);
    }

    #[test]
    fn test_url_long() {
        let input = "https://example.com/files/download.zip";
        let lexems = process(input);
        match &lexems[0] {
            Lexem::Url(url) => assert_eq!(url, input),
            _ => panic!("Expected URL lexem"),
        }
    }

    #[test]
    fn test_multiple_urls() {
        let input = " fdosn   fef   s    https://site1.com   
        an     d http://site2.org a   n  dhtpp http:/ http://www.site3.net";
        let lexems = process(input);

        assert_eq!(lexems.len(), 3);

        match &lexems[0] {
            Lexem::Url(url) => assert_eq!(url, "https://site1.com"),
            _ => panic!("Expected URL lexem"),
        }

        match &lexems[1] {
            Lexem::Url(url) => assert_eq!(url, "http://site2.org"),
            _ => panic!("Expected URL lexem"),
        }

        match &lexems[2] {
            Lexem::Url(url) => assert_eq!(url, "http://www.site3.net"),
            _ => panic!("Expected URL lexem"),
        }
    }
}
