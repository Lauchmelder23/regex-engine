use std::iter::Peekable;

#[derive(Debug)]
pub enum CharacterClass {
    Digit,
    Whitespace,
    WordCharacter
}

#[derive(Debug)]
pub enum CaptureGroupType {
    Normal,
    Anonymous,
    Named(String),
    PositiveLookahead,
    NegativeLookahead,
    PositiveLookbehind,
    NegativeLookbehind
}

#[derive(Debug)]
pub enum Token {
    Character(char),
    CharacterRange(char, char),
    CharacterClass(CharacterClass, bool),

    CharacterClassStart(bool),
    CharacterClassEnd,

    CaptureGroupStart(CaptureGroupType),
    CaptureGroupEnd
}

#[derive(Debug)]
pub struct Lexer<I> 
where
    I: Iterator<Item = char>
{
    scanner: Peekable<I>
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char> 
{
    pub fn new(scanner: I) -> Lexer<I> {
        Lexer {
            scanner: scanner.peekable()
        }
    }

    fn read_integer(&mut self, radix: u32) -> Option<u32> {
        match u32::from_str_radix(self.scanner.by_ref().take_while(|c| c.is_digit(radix)).collect::<String>().as_str(), radix) {
            Ok(result) => Some(result),
            Err(e) => {
                eprintln!("Failed to read DecimalIntgegerLiteral: {e}");
                None
            }
        }
    }

    fn read_hex_number(&mut self, hex_digits: usize) -> Option<u32> {
        let number_string = self.scanner.by_ref().take(hex_digits).collect::<String>();

        if number_string.len() != hex_digits {
            return None;
        }

        match u32::from_str_radix(number_string.as_str(), 16) {
            Ok(result) => Some(result),
            Err(e) => {
                eprintln!("Failed to read DecimalIntgegerLiteral: {e}");
                None
            }
        }
    }

    fn handle_character(&mut self) -> Option<Token> {
        let character = self.scanner.next()?;

        if let Some(dash) = self.scanner.peek() {
            if *dash == '-' {
                return Some(Token::CharacterRange(character, self.scanner.nth(1)?));
            }
        }

        Some(Token::Character(character))
    }

    fn handle_escape(&mut self) -> Option<Token> {
        // discard leading backslash
        self.scanner.next()?;

        let escapee = self.scanner.peek()?.to_ascii_lowercase();

        if "dsw".contains(escapee) {
            return self.handle_character_class_escape();
        }

        if escapee.is_digit(10) {
            return Some(Token::Character(char::from_u32(self.read_integer(10)?)?))
        }

        self.handle_character_escape()
    }

    fn handle_character_class_escape(&mut self) -> Option<Token> {
        match self.scanner.next()? {
            'd' => Some(Token::CharacterClass(CharacterClass::Digit, false)),
            'D' => Some(Token::CharacterClass(CharacterClass::Digit, true)),
            's' => Some(Token::CharacterClass(CharacterClass::Whitespace, false)),
            'S' => Some(Token::CharacterClass(CharacterClass::Whitespace, true)),
            'w' => Some(Token::CharacterClass(CharacterClass::WordCharacter, false)),
            'W' => Some(Token::CharacterClass(CharacterClass::WordCharacter, true)),
            _ => panic!()
        }
    }

    fn handle_character_escape(&mut self) -> Option<Token> {
        match self.scanner.next()? {
            'f' => Some(Token::Character(char::from_u32(12)?)),
            'n' => Some(Token::Character('\n')),
            'r' => Some(Token::Character('\r')),
            't' => Some(Token::Character('\t')),
            'v' => Some(Token::Character(char::from_u32(11)?)),

            'c' => {
                eprintln!("Control sequences are not supported yet");
                None
            }

            'x' => Some(Token::Character(char::from_u32(self.read_hex_number(2)?)?)),
            'u' => Some(Token::Character(char::from_u32(self.read_hex_number(4)?)?)),
            c => {
                eprintln!("Invalid escape sequence '\\{c}'");
                None
            }
        }
    }

    fn handle_character_class_start(&mut self) -> Option<Token> {
        // consume [ character
        self.scanner.next();

        // see if there is a ^ following the [
        let negate = self.scanner.next_if(|&c| c == '^').is_some();
        Some(Token::CharacterClassStart(negate))
    }

    fn handle_capture_group_start(&mut self) -> Option<Token> {
        // consume ( character
        self.scanner.next();

        Some(Token::CaptureGroupStart(
            // Ist das Some(char) oder kann das weg?
            match self.scanner.next_if(|&c| c == '?') {
                None => CaptureGroupType::Normal,
                Some(_) => self.get_capture_group_type()?
            }
        ))
    }

    fn get_capture_group_type(&mut self) -> Option<CaptureGroupType> {
        Some(match self.scanner.next()? {
            ':' => CaptureGroupType::Anonymous,
            '!' => CaptureGroupType::NegativeLookahead,
            '=' => CaptureGroupType::PositiveLookahead,

            _ => {
                eprintln!("Unexpected token after ?");
                return None;
            }
        })
    }
}

impl<I> Iterator for Lexer<I>
where 
    I: Iterator<Item = char> 
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let symbol = self.scanner.peek()?;

        match symbol {
            '-' => todo!(),
            '^' => todo!(),
            '$' => todo!(),
            '\\' => self.handle_escape(),
            '.' => todo!(),
            '*' => todo!(),
            '+' => todo!(),
            '?' => todo!(),
            '(' => self.handle_capture_group_start(),
            ')' => { self.scanner.next(); Some(Token::CaptureGroupEnd)},
            '[' => self.handle_character_class_start(),
            ']' => { self.scanner.next(); Some(Token::CharacterClassEnd) },
            '{' => todo!(),
            '}' => todo!(),
            '|' => todo!(),

            _ => self.handle_character()
        }
    }
}