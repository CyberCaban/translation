pub struct Lexem {
    start: usize,
    end: usize,
}
impl Lexem {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}
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
        let lexem_start = self.idx;
        for expected_char in "http".chars() {
            if expected_char != chars[self.idx] {
                return;
            }
            self.idx += 1;
        }
        if chars[self.idx] == 's' {
            self.idx += 1;
        }

        for expected_char in "://".chars() {
            if expected_char != chars[self.idx] {
                return;
            }
            self.idx += 1;
        }
        while !chars[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        self.parsed_lexems.push(Lexem::new(lexem_start, self.idx));
    }

    pub fn print_lexems(&self, contents: &str) {
        for lexem in &self.parsed_lexems {
            if let Some(l) = contents.get(lexem.start..lexem.end) {
                println!("{}", l);
            }
        }
    }
}
