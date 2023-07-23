use js_sys::Uint8Array;
use std::fmt::Display;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct Line(pub Vec<bool>);

impl Line {
    pub fn from_string(input: String) -> Self {
        Self(input.chars().map(|c| c == '1').collect())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Creates a new line with the center pixel enabled
    pub fn center_enabled(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, false);
        vec[size / 2] = true;
        Self(vec)
    }

    /// Generates the next line given a rule
    pub fn next(&self, rule: u8) -> Self {
        Self(
            self.0
                .iter()
                .enumerate()
                .map(|(i, center)| {
                    let left = if i == 0 { false } else { self.0[i - 1] };

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

/// return the next n lines
/// e.g. wasm_next([0, 1, 0], 30, 3) -> [1, 1, 1, 1, 0, 0, 0, 1, 1]
#[wasm_bindgen]
pub fn wasm_next(line: Uint8Array, rule: u8, n: u32) -> Uint8Array {
    let line = Line(line.to_vec().into_iter().map(|b| b == 1).collect());
    Uint8Array::from(
        (0..n)
            .scan(line, |line, _| {
                let next = line.next(rule);
                *line = next.clone();
                Some(next)
            })
            .flat_map(|line| line.0.into_iter().map(|b| if b { 1 } else { 0 }))
            .collect::<Vec<u8>>()
            .as_slice(),
    )
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.0 {
            write!(f, "{}", if *b { "█" } else { " " })?;
        }
        Ok(())
    }
}
