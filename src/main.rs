mod day1;
mod helper;

fn main() {
    let base_address = String::from("inputs/");

    println!("Day 1 part 1 answer {}", day1::part1(&base_address));
    println!("Day 1 part 2 answer {}", day1::part2(&base_address));
}
