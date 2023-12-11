mod cubes;
mod part1;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1 answer: {}", part1::run_part1(INPUT));
}
