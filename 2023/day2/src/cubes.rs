#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct CubeCount {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl CubeCount {
    pub fn from_str(round: &str) -> Self {
        round
            .split(',')
            .fold(CubeCount::default(), |mut acc, cubes| {
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

    pub fn impossible(&self, test_subject: Self) -> bool {
        test_subject.red > self.red
            || test_subject.blue > self.blue
            || test_subject.green > self.green
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
