pub mod node;
mod parser;

use parser::ParserResult;

/// Parses a regex pattern into an expression tree
/// 
/// # Arguments
/// 
/// * `regex` - A string slice that holds a regex pattern
/// 
/// # Examples
/// ```
/// let regex = regex::new("Hell.* world");
/// ```
pub fn new(pattern: &str) -> ParserResult {
    parser::parse(pattern)
}