use std::iter::Peekable;

use crate::lexer::Lexer;

#[derive(Debug)]
pub struct Parser<I>
where 
    I: Iterator<Item = char> 
{
    lexer: Peekable<Lexer<I>>
} 

impl<I> Parser<I>
where
    I: Iterator<Item = char>
{
    fn new(scanner: I) -> Parser<I> {
        Parser {
            lexer: Lexer::new(scanner).peekable()
        }
    }

    fn parse(self) -> Result<(), String> {
        self.lexer.for_each(|token| println!("{token:?}",));
        Ok(())
    }
}

pub fn parse_string(input: &str) -> Result<(), String> {
    let scanner = input.chars();

    Parser::new(scanner).parse()?;

    Ok(())
}