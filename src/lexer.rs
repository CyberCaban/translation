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
        while !chars[self.idx].is_ascii_whitespace() {
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
