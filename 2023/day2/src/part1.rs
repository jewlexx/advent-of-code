#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
struct CubeCount {
    red: u8,
    green: u8,
    blue: u8,
}

impl CubeCount {
    fn from_str(round: &str) -> Self {
        round.split(',').fold(CubeCount::default(), |acc, cubes| {
            let mut acc = acc;
            let (count, colour) = cubes.trim().split_once(' ').unwrap();

            let count = count.parse::<u8>().expect("valid integer");
            match colour {
                "red" => acc.red += count,
                "green" => acc.green += count,
                "blue" => acc.blue += count,
                _ => unreachable!(),
            }

            acc
        })
    }
}

impl PartialOrd for CubeCount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CubeCount {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use core::cmp::Ordering;
        if self == other {
            Ordering::Equal
        } else if let Ordering::Less = self.red.cmp(&other.red) {
            Ordering::Less
        } else if let Ordering::Less = self.green.cmp(&other.green) {
            Ordering::Less
        } else {
            self.blue.cmp(&other.blue)
        }
    }
}

const ACTUAL_CUBES: CubeCount = CubeCount {
    red: 12,
    green: 13,
    blue: 14,
};

pub fn run_part1(input: &str) -> usize {
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
        .filter(|(i, v)| {
            if v.iter().any(|cubecount| ACTUAL_CUBES <= *cubecount) {
                println!("Game {i} impossible");
                false
            } else {
                println!("Game {i} possible");
                true
            }
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

        let sum = run_part1(input);

        assert_eq!(sum, 8);
    }
}
