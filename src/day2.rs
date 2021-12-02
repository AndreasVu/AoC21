use crate::helper;
use std::str::FromStr;

pub fn part1(base_address: &String) -> i32 {
    let file_path = base_address.clone() + "day2.txt";
    let lines = helper::get_file_as_vec(&file_path);
    let mut depth = 0;
    let mut forward = 0;

    for movement in parse_lines_to_struct(&lines){
        match movement.direction {
            Direction::Up => depth -= movement.units,
            Direction::Down => depth += movement.units,
            Direction::Forward => forward += movement.units
        };
    }

    return depth * forward;
}

pub fn part2(base_address: &String) -> i32 {
    let file_path = base_address.clone() + "day2.txt";
    let lines = helper::get_file_as_vec(&file_path);

    let mut depth = 0;
    let mut forward = 0;
    let mut aim = 0;

    for movement in parse_lines_to_struct(&lines){
        match movement.direction {
            Direction::Up => aim -= movement.units,
            Direction::Down => aim += movement.units,
            Direction::Forward => {
                forward += movement.units;
                depth += aim * movement.units;
            }
        };
    }

    return depth * forward;
}

struct Move {
    direction: Direction,
    units: i32
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Forward,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "up"  => Ok(Direction::Up),
            "down"  => Ok(Direction::Down),
            "forward"  => Ok(Direction::Forward),
            _ => Err(()),
        }
    }
}

fn parse_lines_to_struct(lines: &Vec<String> ) -> Vec<Move> {
    let mut moves = Vec::<Move>::new();

    for line in lines {
        let split_string: Vec::<&str> = line.split(" ").collect();

        moves.push(Move {
            direction: Direction::from_str(split_string[0]).unwrap(),
            units: split_string[1].parse::<i32>().unwrap()
        })
    }

    return moves;
}
