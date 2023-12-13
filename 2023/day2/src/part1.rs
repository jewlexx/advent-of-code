use super::cubes::CubeCount;

const ACTUAL_CUBES: CubeCount = CubeCount {
    red: 12,
    green: 13,
    blue: 14,
};

pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (game, rounds) = line.split_once(": ").unwrap();

            let game_id = game["Game ".len()..].parse::<usize>().unwrap();

            (game_id, rounds)
        })
        .map(|(game_id, rounds)| {
            (
                game_id,
                rounds
                    .split(';')
                    .map(CubeCount::from_str)
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(_, v)| {
            v.iter()
                .any(|cubecount| ACTUAL_CUBES.impossible(*cubecount))
        })
        .map(|(i, _)| i)
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

        let sum = run(input);

        assert_eq!(sum, 8);
    }
}
