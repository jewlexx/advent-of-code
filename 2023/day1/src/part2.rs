#[rustfmt::skip]
const POSSIBLE_DIGITS: [&str; 18] = [
    // Regular ascii digits
    "1", "2", "3", "4", "5", "6", "7", "8", "9",
    // Numbers as words
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Debug, Copy, Clone)]
pub struct EndOffset {
    // TODO: The start might be pointless
    pub start: isize,
    pub end: isize,
}

impl EndOffset {
    /// The offsets to apply after recognizing each digit
    /// to ensure no digits are missed that have overlapping letters
    const END_OFFSETS: [EndOffset; 18] = [
        // Regular ascii digits
        EndOffset::new(0, 0),
        EndOffset::new(0, 0),
        EndOffset::new(0, 0),
        EndOffset::new(0, 0),
        EndOffset::new(0, 0),
        EndOffset::new(0, 0),
        EndOffset::new(0, 0),
        EndOffset::new(0, 0),
        EndOffset::new(0, 0),
        // Numbers as words
        EndOffset::new(0, -1),
        EndOffset::new(0, -1),
        EndOffset::new(0, -1),
        EndOffset::new(0, 0),
        EndOffset::new(0, -1),
        EndOffset::new(0, 0),
        EndOffset::new(0, 0),
        EndOffset::new(1, -1),
        EndOffset::new(0, -1),
    ];

    pub const fn new(start: isize, end: isize) -> Self {
        Self { start, end }
    }

    pub const fn get(i: usize) -> Self {
        Self::END_OFFSETS[i]
    }

    pub fn apply(self) {
        todo!()
    }
}

pub fn str_to_digit(str: &str) -> Option<(usize, EndOffset)> {
    POSSIBLE_DIGITS
        .iter()
        .position(|s| *s == str)
        .map(|i| ((i % 9) + 1, EndOffset::get(i)))
}

pub fn run(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(char::is_ascii_digit)
                .collect::<Vec<_>>()
        })
        .filter_map(|line_numbers| {
            let first = line_numbers.first()?;
            let last = line_numbers.last()?;
            let combined = format!("{first}{last}");

            Some(combined.parse::<u32>().unwrap())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let sum = run("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");

        assert_eq!(sum, 281);
    }

    #[test]
    fn test_str_to_digit() {
        for (i, digit) in POSSIBLE_DIGITS.into_iter().enumerate() {
            // Find the actual number, from the index
            let number = (i % 9) + 1;

            assert_eq!(number, str_to_digit(digit).unwrap().0);
        }
    }
}
