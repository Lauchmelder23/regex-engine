pub type Branch = Box<Node>;

#[derive(Debug, Clone)]
pub enum Node {
    Empty(bool),

    PatternCharacter(u32),
    AnyCharacter,

	Disjunction(Branch, Branch),
    Term(Branch, Quantifier, Branch)
}

#[derive(Debug, Copy, Clone)]
pub struct Quantifier {
    min: usize,
    max: usize,
    greedy: bool
}

impl Quantifier {
    pub fn new(min: usize, max: usize, greedy: bool) -> Quantifier {
        Quantifier { min: min, max: max, greedy: greedy }
    }
}

impl Node {
    pub fn test(&self, input: &str) -> usize {
        self.eval(&input.chars().map(|c| u32::from(c)).collect(), 0).unwrap_or(usize::MAX)
    }

    fn eval(&self, data: &Vec<u32>, pos: usize) -> Option<usize> {
        match self {
            Self::Empty(b) => {
                if *b {
                    Some(pos)
                } else {
                    None 
                }
            },

 
            Self::PatternCharacter(c) => {
                if pos >= data.len() {
                    return None;
                }

                if &data[pos] == c {
                    Some(pos + 1)
                } else {
                    None
                }
            }

            Self::AnyCharacter => {
                if pos < data.len() {
                    Some(pos + 1)
                } else {
                    None
                }
            }

            Self::Disjunction(left, right) => {
                match left.eval(data, pos) {
                    Some(new_pos) => Some(new_pos),
                    None => right.eval(data, pos)
                }
            }

            Self::Term(atom, quantifier, remainder) => {
                self.eval_quantifier(pos, atom, quantifier, remainder, data, pos)
            }
        }
    }

    fn eval_quantifier(
        &self, 
        parent_pos: usize,
        atom: &Box<Node>, 
        quantifier: &Quantifier, 
        remainder: &Box<Node>, 
        data: &Vec<u32>, 
        mut pos: usize
    ) -> Option<usize> {
        let mut repetitions = pos - parent_pos;
        while repetitions + 1 < quantifier.min {
            pos = atom.eval(data, pos)?;
            repetitions += 1;
        }

        match quantifier.greedy {
            true => {
                // Try matching atom as often as possible
                let mut match_history = vec![];
                let old_pos = pos;
                for _ in 0..quantifier.max {
                    match atom.eval(data, pos) {
                        None => break,
                        Some(new_pos) => {
                            match_history.push(new_pos);
                            pos = new_pos;
                        }
                    }
                }

                // If array size is smaller than min, matching failed
                if match_history.len() < quantifier.min {
                    return None;
                }

                // Try matching remainder at positions, starting from the back
                for matching_pos in match_history.iter().rev() {
                    //print!("{} -> ", matching_pos);
                    match remainder.eval(data, *matching_pos) {
                        None => {},
                        Some(new_pos) => { 
                            return Some(new_pos); 
                        }
                    }
                }

                if quantifier.min != 0 {
                    return None;
                }

                remainder.eval(data, old_pos)
            },
            false => todo!()
        }
    }
}