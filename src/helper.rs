use std::fs;

pub fn get_file_string(path: &str)-> std::string::String {
    return fs::read_to_string(path).expect("Unable to read file");
}

pub fn get_file_as_vec(path: &str) -> Vec<i32> {
    let file_string = get_file_string(path);

    let lines: Vec<i32> = file_string.lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    return lines;
}
