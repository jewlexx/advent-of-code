// TODO: Part 2 not completed

const INPUT: &str = include_str!("../input.txt");

mod part1;
mod part2;

fn main() {
    println!("Answer to part 1 is: {}", part1::run(INPUT));
    println!("Answer to part 2 is: {}", part2::run(INPUT));
}
