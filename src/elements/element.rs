use std::str::Chars;

pub type ParseResult<T> = Option<(T, usize)>;

pub trait Element {
	fn parse(expr: &str, pos: usize) -> ParseResult<Self> where Self: Sized;
	fn test(&self, it: &mut Chars) -> bool;
}