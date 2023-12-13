use std::ops::Range;

use regex::Regex;

pub struct Patterns {
    symbols: Regex,
    numbers: Regex,
}

impl Patterns {
    pub const SYMBOLS: &'static str = r"[^\d\.]";
    pub const NUMBERS: &'static str = r"[^\d\.]";

    pub fn new() -> Self {
        Self {
            symbols: Regex::new(Self::SYMBOLS).expect("valid regex"),
            numbers: Regex::new(Self::NUMBERS).expect("valid regex"),
        }
    }

    pub fn symbols_matrix(&self, input: &str) -> Vec<Vec<usize>> {
        input
            .lines()
            .map(|line| {
                self.symbols
                    .find_iter(line)
                    // `mat.start()` should work as each symbol is only 1 character (hopefully)
                    .map(|mat| mat.start())
                    .collect()
            })
            .collect()
    }

    pub fn numbers_matrix(&self, input: &str) -> Vec<Vec<Range<usize>>> {
        input
            .lines()
            .map(|line| {
                self.numbers
                    .find_iter(line)
                    // `mat.start()` should work as each symbol is only 1 character (hopefully)
                    .map(|mat| mat.range())
                    .collect()
            })
            .collect()
    }
}

/// Iterate over two iterators at once
#[derive(Debug, Copy, Clone)]
pub struct SimIter<A, B>(A, B);

impl<A: Iterator<Item = AT>, B: Iterator<Item = BT>, AT, BT> Iterator for SimIter<A, B> {
    type Item = (AT, BT);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.0.next(), self.1.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            (None, None) => None,
            _ => panic!("One iterator ran before the other"),
        }
    }
}

pub fn run(input: &str) -> usize {
    let patterns = Patterns::new();

    let number_matrix = patterns.numbers_matrix(input);
    let symbol_matrix = patterns.symbols_matrix(input);

    let matrix_line_iter = SimIter(number_matrix.into_iter(), symbol_matrix.into_iter());

    for (numbers, symbols) in matrix_line_iter {}

    todo!()
}
