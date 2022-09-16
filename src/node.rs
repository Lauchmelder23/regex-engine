use std::{str::Chars, iter::Peekable};

pub type Branch = Box<Node>;

#[derive(Debug)]
pub enum Node {
    Empty(bool),

    PatternCharacter(u32),
    AnyCharacter,

	Disjunction(Branch, Branch),
	Alternative(Branch, Branch),
    Atom(Branch, Quantifier),
}

#[derive(Debug)]
pub enum Quantifier {
    One,
    OneOrNone,
    OneOrMore,
    NoneOrMore,
    Range(usize, usize)
}

impl Quantifier {
    fn eval(&self, atom: &Branch, data: &Vec<u32>, pos: &mut usize) -> bool {
        match self {
            Self::One => atom.eval(data, pos),

            Self::OneOrNone => {
                let old_pos = pos.clone();
                if !atom.eval(data, pos) {
                    *pos = old_pos;
                }

                true
            }

            Self::OneOrMore => {
                if !atom.eval(data, pos) {
                    return false;
                }

                loop {
                    let old_pos = pos.clone();
                    if !atom.eval(data, pos) {
                        *pos = old_pos;
                        break
                    }
                }

                true
            }

            Self::NoneOrMore => {
                loop {
                    let old_pos = pos.clone();
                    if !atom.eval(data, pos) {
                        *pos = old_pos;
                        break
                    }
                }

                true
            },

            _ => todo!()
        }
    }
}

impl Node {
    pub fn test(&self, input: &str) -> bool {
        self.eval(&input.chars().map(|c| u32::from(c)).collect(), &mut 0)
    }

    fn eval(&self, data: &Vec<u32>, pos: &mut usize) -> bool {
        match self {
            Self::Empty(b) => *b,


            Self::PatternCharacter(c) => {
                let cur_pos = *pos;
                *pos += 1;
                &data[cur_pos] == c
            }

            Self::AnyCharacter => {
                *pos += 1;
                true
            }

            Self::Disjunction(left, right) => {
                let mut orig_pos = *pos;
                left.eval(data, pos) || right.eval(data, &mut orig_pos)
            }

            Self::Alternative(left, right) => {
                left.eval(data, pos) && right.eval(data, pos)
            },

            Self::Atom(left, quantifier) => {
                quantifier.eval(left, data, pos)
            }
        }
    }
}