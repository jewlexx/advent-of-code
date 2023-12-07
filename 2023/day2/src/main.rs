const INPUT: &str = include_str!("../input.txt");

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
        if let core::cmp::Ordering::Less = self.red.cmp(&other.red) {
            return core::cmp::Ordering::Less;
        }
        if let core::cmp::Ordering::Less = self.green.cmp(&other.green) {
            return core::cmp::Ordering::Less;
        }
        self.blue.cmp(&other.blue)
    }
}

const ACTUAL_CUBES: CubeCount = CubeCount {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() {
    let games = INPUT
        .lines()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|game| game.split(';').map(CubeCount::from_str).collect::<Vec<_>>())
        .enumerate()
        .map(|(i, v)| (i, v))
        .filter(|(_, v)| v.iter().any(|cubecount| ACTUAL_CUBES >= *cubecount))
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    println!("{games}");
}
