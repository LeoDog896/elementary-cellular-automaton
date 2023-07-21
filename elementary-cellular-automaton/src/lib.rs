use bitvec::prelude::*;
use std::fmt::Display;

pub struct Line(pub BitVec);

impl Line {
    pub fn from_string(input: String) -> Self {
        Self(input.chars().map(|c| c == '1').collect())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Creates a new line with the center pixel enabled
    pub fn center_enabled(size: usize) -> Self {
        let mut vec = bitvec![0; size];
        vec.set(size / 2, true);
        Self(vec)
    }

    /// Generates the next line given a rule
    pub fn next(&self, rule: u8) -> Self {
        Self(
            self.0
                .iter()
                .enumerate()
                .map(|(i, center)| {
                    let left = if i == 0 {
                        false
                    } else {
                        self.0[i - 1]
                    };

                    let right = if i == self.0.len() - 1 {
                        false
                    } else {
                        self.0[i + 1]
                    };
                    
                    let index = u8::from(left) << 2 | u8::from(*center) << 1 | u8::from(right);
                    rule >> index & 1 == 1
                })
                .collect(),
        )
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.0 {
            write!(f, "{}", if *b { "█" } else { " " })?;
        }
        Ok(())
    }
}