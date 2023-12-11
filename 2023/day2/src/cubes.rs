#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct CubeCount {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl CubeCount {
    pub fn from_str(round: &str) -> Self {
        round
            .split(',')
            .fold(CubeCount::default(), |mut acc, cubes| {
                let (count, colour) = cubes.trim().split_once(' ').unwrap();

                let count = count.parse::<usize>().expect("valid integer");
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

    /// Merges with `other`, by replacing all elements of `self` where `other`'s counterparts are greater
    pub fn merge(mut self, other: Self) -> Self {
        if other.red > self.red {
            self.red = other.red;
        }
        if other.green > self.green {
            self.green = other.green;
        }
        if other.blue > self.blue {
            self.blue = other.blue;
        }
        self
    }

    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
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
