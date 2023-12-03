const INPUT: &str = include_str!("../input.txt");

fn main() {
    let calibration_value_sum = INPUT
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
        .sum::<u32>();

    println!("The sum of all the calibration values is {calibration_value_sum}");
}
