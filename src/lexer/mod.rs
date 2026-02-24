#[cfg(test)]
mod tests;

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
    // инициализация (конструктор)
    pub fn new() -> Lexer {
        Lexer {
            idx: 0,
            parsed_lexems: vec![],
        }
    }
    pub fn lex(&mut self, contents: &str) {
        // считываем из строки все символы в массив символов
        let chars: Vec<char> = contents.chars().collect();

        while self.idx < chars.len() {
            let mut ch = chars[self.idx];

            // пропуск пробельных символов
            while ch.is_ascii_whitespace() {
                self.idx += 1;
                if self.idx >= chars.len() {
                    return;
                }
                ch = chars[self.idx];
            }

            match ch {
                // если очередной символ 'h', то начинаем парсить url
                'h' => {
                    self.parse_url(&chars);
                }
                // иначе пропускаем символ
                _ => {
                    self.idx += 1;
                }
            }
        }
    }
    fn parse_url(&mut self, chars: &[char]) {
        // объявление enum и struct внутри тела функции сделано для того, чтобы ограничить область видимости,
        // т.е. компилятор создаст тип один раз во время компиляции
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum UrlState {
            Start,
            H,
            HT,
            HTT,
            HTTP,
            HTTPS,
            Colon,
            ColonSlash,
            Done,
        }
        let start = self.idx;
        // начальное состояние
        let mut state = UrlState::Start;
        while self.idx < chars.len() {
            let ch = chars[self.idx];
            // переходы автомата:
            // сравниваем состояние и текущий символ, а потом присваиваем в текущее состояние
            state = match (state, ch) {
                // шаблон сопоставления:
                // (состояние, символ) => новое состояние
                (UrlState::Start, 'h') => UrlState::H,
                (UrlState::H, 't') => UrlState::HT,
                (UrlState::HT, 't') => UrlState::HTT,
                (UrlState::HTT, 'p') => UrlState::HTTP,
                (UrlState::HTTP, 's') => UrlState::HTTPS,
                (UrlState::HTTP, ':') => UrlState::Colon,
                (UrlState::HTTPS, ':') => UrlState::Colon,
                (UrlState::Colon, '/') => UrlState::ColonSlash,
                (UrlState::ColonSlash, '/') => UrlState::Done,
                // здесь происходит проверка символа на пробельный,
                // если так, то останавливаем считывание
                (UrlState::Done, c) if c.is_ascii_whitespace() => break,
                // нижнее подчеркивание означает любой
                (UrlState::Done, _) => UrlState::Done,
                // все не подходящее под шаблон уходит в ловушку
                _ => return,
            };
            self.idx += 1;
        }
        // собираем строку из символов с индекса start по индекс self.idx не включая
        let url_lexem = chars[start..self.idx].iter().collect();
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
