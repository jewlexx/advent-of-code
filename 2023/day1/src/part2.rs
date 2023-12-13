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

    pub fn apply(self, length: usize) -> usize {
        let mut length = length as isize;
        length += self.end;

        length as usize
    }
}

pub fn str_to_digit(str: &str) -> Option<(usize, EndOffset)> {
    POSSIBLE_DIGITS
        .iter()
        .position(|s| *s == str)
        .map(|i| ((i % 9) + 1, EndOffset::get(i)))
}

// This does not consider digits that are substrings
pub fn find_digits_in_line_old(line: &str) -> Vec<usize> {
    let chars = line.chars();

    let mut current_string = String::new();
    let mut found_digits = vec![];

    for c in chars {
        current_string.push(c);

        if let Some((digit, end_offset)) = str_to_digit(&current_string) {
            found_digits.push(digit);
            let new_start = end_offset.apply(current_string.len());
            current_string = String::from(&current_string[new_start..]);
        }
    }

    found_digits
}

pub fn find_digits_in_line(line: &str) -> Vec<usize> {
    // Obviously this solution is not super optimized
    // There are obvious improvements to be made where we could filter all digits that are longer than the string we have available
    // But for this case, it's not that important
    let mut found_digits = POSSIBLE_DIGITS
        .into_iter()
        .enumerate()
        .filter_map(|(i, digit)| line.find(digit).map(|pos| ((i % 9) + 1, pos)))
        .collect::<Vec<(usize, usize)>>();

    found_digits.sort_by_key(|(_, i)| *i);

    found_digits.into_iter().map(|(number, _)| number).collect()
}

pub fn run(input: &str) -> u32 {
    input
        .lines()
        .map(find_digits_in_line)
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

    #[test]
    fn test_digits_in_line() {
        let digits = find_digits_in_line("two1nine");
        assert_eq!(digits, [2, 1, 9])
    }
}
