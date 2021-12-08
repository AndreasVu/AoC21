use crate::helper;

pub fn part2(base_address: &String) -> isize {
    let file_path = base_address.clone() + "day3.txt";
    let lines = helper::get_file_as_vec(&file_path);
    let mut index = 0;

    let mut co2_vec = lines.clone();
    let mut ox_vec = lines.clone();

    let mut ox_found = false;
    let mut co2_found = false;

    while !(ox_found && co2_found) {
         if !ox_found {
            let ones = (&ox_vec).into_iter().fold(0 as u32, |accum, line| {
                if line.chars().nth(index).unwrap() == '1' {
                    return accum + 1;
                }
                
                return accum;
            });
            let most_common: char;

            if ones as f32 >= ox_vec.len() as f32 / 2.0 {
                most_common = '1';
            } else {
                most_common = '0';
            }

            ox_vec = ox_vec.clone().into_iter().filter(|line| line.chars().nth(index).unwrap() == most_common).collect();

            if ox_vec.len() == 1 {
                ox_found = true;
            }
        }

        if !co2_found {
            let ones = (&co2_vec).into_iter().fold(0, |accum, line| {
                if line.chars().nth(index).unwrap() == '1' {
                    return accum + 1;
                }
                
                return accum;
            });
            let least_common: char;

            if ones as f32 >= co2_vec.len() as f32 / 2.0 {
                least_common = '0';
            } else {
                least_common = '1';
            }

            co2_vec = co2_vec.clone().into_iter().filter(|line| line.chars().nth(index).unwrap() == least_common).collect();

            if co2_vec.len() == 1 {
                co2_found = true;
            }
        }

        index += 1;
    }

    return into_binary(&ox_vec.into_iter().next().unwrap()) * into_binary(&co2_vec.into_iter().next().unwrap());
}

fn into_binary(binary: &String) -> isize {
    return isize::from_str_radix(binary, 2).unwrap();
}