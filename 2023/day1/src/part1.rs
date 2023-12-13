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
}
