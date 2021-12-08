use crate::helper;

type Board = Vec<Vec<Number>>;

#[derive(Clone, Copy)]
struct Number {
  value: u32,
  selected: bool,
}

pub fn part1(base_address: &String) -> u32 {
  let file = helper::get_file_string(&(base_address.clone() + "day4.txt"));

  let mut iterator = file.split("\n\n").into_iter();

  let bingo_numbers: Vec<u32> = iterator
    .next()
    .unwrap()
    .split(",")
    .map(|num| num.parse().unwrap())
    .collect();

  let mut boards: Vec<Board> = iterator
    .map(|board| {
      return board
        .split("\n")
        .map(|line| {
          line
            .split_whitespace()
            .map(|number| Number {
              value: number.parse().unwrap(),
              selected: false,
            })
            .collect()
        })
        .collect();
    })
    .collect();

  for number in bingo_numbers {
    for board in &mut boards {
      update_board(board, number);
      if has_won(board) {
        let sum = board
          .into_iter()
          .flatten()
          .filter(|number| !number.selected)
          .fold(0, |acc, number| acc + number.value);
        return sum * number;
      }
    }
  }

  panic!("ÆÆÆÆÆÆÆÆÆÆ FANT IKKE SVARET");
}

fn update_board(board: &mut Board, bingo_number: u32) {
  for row in board {
    for number in row
      .into_iter()
      .filter(|number| number.value == bingo_number)
    {
      number.selected = true;
    }
  }
}

fn has_won(board: &Board) -> bool {
  for row in board {
    if all_selected(&row) {
      return true;
    }
  }

  for column in 0..board.len() {
    let column_values: Vec<Number> = board.into_iter().map(|row| row[column]).collect();
    if all_selected(&column_values) {
      return true;
    }
  }

  return false;
}

fn all_selected(numbers: &Vec<Number>) -> bool {
  return numbers.into_iter().all(|number| number.selected);
}
