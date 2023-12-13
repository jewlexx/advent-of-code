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
