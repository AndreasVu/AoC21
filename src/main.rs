mod day1;
mod day2;
mod day3;
mod day4;
mod helper;

fn main() {
    let base_address = String::from("inputs/");

    println!("Day 1 part 1 answer {}", day1::part1(&base_address));
    println!("Day 1 part 2 answer {}", day1::part2(&base_address));
    println!("-------");

    println!("Day 2 part 1 answer {}", day2::part1(&base_address));
    println!("Day 2 part 2 answer {}", day2::part2(&base_address));

    println!("Day 3 part 2 answer {}", day3::part2(&base_address));

    println!("Day 4 part 1 answer {}", day4::part1(&base_address));
}
