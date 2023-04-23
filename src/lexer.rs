use std::iter::Peekable;

#[derive(Debug)]
pub enum Token {
    Character(char),
    CharacterRange(char, char)
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

    fn handle_character(&mut self) -> Option<Token> {
        let character = self.scanner.next()?;

        if let Some(dash) = self.scanner.peek() {
            if *dash == '-' {
                return Some(Token::CharacterRange(character, self.scanner.nth(1)?));
            }
        }

        Some(Token::Character(character))
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
            '\\' => todo!(),
            '.' => todo!(),
            '*' => todo!(),
            '+' => todo!(),
            '?' => todo!(),
            '(' => todo!(),
            ')' => todo!(),
            '[' => todo!(),
            ']' => todo!(),
            '{' => todo!(),
            '}' => todo!(),
            '|' => todo!(),

            _ => self.handle_character()
        }
    }
}