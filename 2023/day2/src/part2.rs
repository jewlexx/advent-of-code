use crate::cubes::*;

pub fn run_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|rounds| {
            rounds
                .split(';')
                .map(CubeCount::from_str)
                .collect::<Vec<_>>()
        })
        .map(|v| v.into_iter().fold(CubeCount::default(), CubeCount::merge))
        .map(|v| v.power())
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let sum = run_part2(input);

        assert_eq!(sum, 2286);
    }
}
