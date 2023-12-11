mod cubes;
mod part1;
mod part2;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1 answer: {}", part1::run_part1(INPUT));
    println!("Part 2 answer: {}", part2::run_part2(INPUT));
}
