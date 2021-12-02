use crate::helper;

pub fn part1(base_address: &String) -> String {
    let file_path = base_address.clone() + "day1.txt";

    let mut last_number = i32::MAX;
    let mut counter = 0;

    for number in helper::get_file_as_vec_int(&file_path) {
        if number > last_number {
            counter += 1;
        }

        last_number = number;
    }

    return counter.to_string();
}

pub fn part2(base_address: &String) -> String {
    let file_path = base_address.clone() + "day1.txt";
    let lines = helper::get_file_as_vec_int(&file_path);

    let mut prev_value = i32::MAX;
    let mut counter = 0;
    for i in 3..(lines.len() + 1) {
        let sum = lines[i-3..i].into_iter().cloned().reduce(|a, b| a + b).unwrap();

        if sum > prev_value {
            counter += 1;
        }

        prev_value = sum;
    }

    return counter.to_string();
}
