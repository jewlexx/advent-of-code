const INPUT: &str = include_str!("../input.txt");

fn i_to_num(i: usize) -> usize {
    (i % (POSSIBLE_DIGITS.len() / 2)) + 1
}

#[rustfmt::skip]
pub const POSSIBLE_DIGITS: [&str; 18] = [
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

pub fn sum_lines(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let locations = POSSIBLE_DIGITS
                .iter()
                .enumerate()
                .filter_map(|(i, digit)| line.find(digit).map(|pos| (pos, i_to_num(i))))
                .collect::<Vec<_>>();

            let first = locations.iter().min_by_key(|d| d.0).unwrap();
            let last = locations.iter().max_by_key(|d| d.0).unwrap();

            if locations.len() == 1 {
                assert_eq!(first, last);
            } else {
                assert_ne!(first, last);
            }

            let first = first.1;
            let last = last.1;
            let combined = format!("{first}{last}");

            combined.parse::<u32>().unwrap()
        })
        .sum::<u32>()
}

fn main() {
    let calibration_value_sum = sum_lines(INPUT);

    println!("The sum of all the calibration values is {calibration_value_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let sum = sum_lines(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        assert_eq!(sum, 281);
    }

    #[test]
    fn test_digits() {
        for (i, digit) in POSSIBLE_DIGITS.iter().enumerate() {
            let predicted = i_to_num(i);
            let actual = match *digit {
                "1" | "one" => 1,
                "2" | "two" => 2,
                "3" | "three" => 3,
                "4" | "four" => 4,
                "5" | "five" => 5,
                "6" | "six" => 6,
                "7" | "seven" => 7,
                "8" | "eight" => 8,
                "9" | "nine" => 9,
                _ => unreachable!(),
            };

            assert_eq!(predicted, actual);
        }
    }
}
