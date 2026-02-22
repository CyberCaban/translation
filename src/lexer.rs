#[derive(Debug, PartialEq)]
pub enum Lexem {
    Url(String),
    Number(f64),
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
                // number start
                c if c.is_ascii_digit() || c == '.' || c == '-' => {
                    self.parse_number(&chars);
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
    fn parse_number(&mut self, chars: &[char]) {
        enum NumberState {
            Start,
            Sign,
            Integer,
            DecimalPoint,
            Fraction,
            Exponent,
        }
        let mut state = NumberState::Start;
        let mut num: f64 = 0.0;
        let mut sign = 1.0;
        let mut exponent = 0;
        let mut exponent_sign = 1;
        let mut has_decimal = false;
        let mut fraction_divider = 1.0;
        while self.idx < chars.len() {
            let c = chars[self.idx];
            match state {
                NumberState::Start => match c {
                    '-' => {
                        if sign < 0.0 {
                            // already seen minus
                            return;
                        }
                        sign = -1.0;
                        self.idx += 1;
                        state = NumberState::Sign;
                    }
                    ch if ch.is_ascii_digit() => {
                        let digit = ch.to_digit(10).unwrap() as f64;
                        num = num * 10.0 + digit;
                        self.idx += 1;
                        state = NumberState::Integer;
                    }
                    '.' => {
                        self.idx += 1;
                        state = NumberState::DecimalPoint;
                    }
                    _ => break,
                },
                NumberState::Sign => match c {
                    ch if ch.is_ascii_digit() => {
                        let digit = ch.to_digit(10).unwrap() as f64;
                        num = num * 10.0 + digit;
                        self.idx += 1;
                        state = NumberState::Integer;
                    }
                    _ => break,
                },
                NumberState::Integer => match c {
                    ch if ch.is_ascii_digit() => {
                        let digit = ch.to_digit(10).unwrap() as f64;
                        num = num * 10.0 + digit;
                        self.idx += 1;
                        state = NumberState::Integer;
                    }
                    '.' => {
                        self.idx += 1;
                        state = NumberState::DecimalPoint;
                    }
                    'e' | 'E' => {
                        self.idx += 1;
                        state = NumberState::Exponent;
                    }
                    _ => break,
                },
                NumberState::DecimalPoint => {
                    if c.is_ascii_digit() {
                        let digit = c.to_digit(10).unwrap() as f64;
                        num = num * 10.0 + digit;
                        self.idx += 1;
                        state = NumberState::Fraction;
                    } else {
                        break;
                    }
                }
                NumberState::Fraction => match c {
                    ch if ch.is_ascii_digit() => {
                        let digit = ch.to_digit(10).unwrap() as f64;
                        fraction_divider *= 10.0;
                        num += digit / fraction_divider;
                        self.idx += 1;
                        state = NumberState::Integer;
                    }
                    'e' | 'E' => {
                        self.idx += 1;
                        state = NumberState::Exponent;
                    }
                    _ => break,
                },
                NumberState::Exponent => match c {
                    '-' => {
                        exponent_sign = -1;
                        self.idx += 1;
                    }
                    '+' => {
                        self.idx += 1;
                    }
                    ch if c.is_ascii_digit() => {
                        let digit = c.to_digit(10).unwrap() as i32;
                        exponent = exponent * 10 + digit;
                        self.idx += 1;
                    }
                    _ => break,
                },
            }
        }
        let result = sign * num * (10.0f64).powi(exponent_sign * exponent);
        self.parsed_lexems.push(Lexem::Number(result));
    }

    pub fn print_lexems(&self) {
        for lexem in &self.parsed_lexems {
            match lexem {
                Lexem::Url(url) => {
                    println!("Url: {:?}", url);
                }
                Lexem::Number(num) => {
                    println!("Number: {:?}", num)
                }
                _ => {}
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
