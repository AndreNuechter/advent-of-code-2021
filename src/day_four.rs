use std::fmt;

#[derive(Clone)]
struct Cell {
    value: u32,
    marked: bool,
}

pub fn day_four(input: &str) {
    let input: Vec<&str> = input.split("\n\n").collect();
    let drawn_numbers: Vec<u32> = input[0]
        .split(",")
        .map(|num_str| num_str.parse::<u32>().unwrap())
        .collect();
    let mut boards: Vec<Vec<Vec<Cell>>> = input[1..]
        .iter()
        .map(|board| {
            board
                .split("\n")
                .map(|row| {
                    row.split_whitespace()
                        .map(|cell| Cell {
                            value: cell.parse::<u32>().unwrap(),
                            marked: false,
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    part_one(&drawn_numbers, &mut boards);
    part_two(&drawn_numbers, &mut boards);
}

fn part_two(drawn_numbers: &Vec<u32>, boards: &mut Vec<Vec<Vec<Cell>>>) {
    for num in drawn_numbers.iter() {
        let mut winners = Vec::new();

        *boards = boards
            .into_iter()
            .map(|mut board| mark_number(&mut board, *num).to_vec())
            .collect();

        for (j, board) in boards.iter().enumerate() {
            if check_for_win(&board) {
                if boards.len() - winners.len() > 1 {
                    winners.push(j);
                } else {
                    let sum = sum_unmarked(&board);
                    println!("pt 2: found the loser");
                    println!("winning score: {} x {} = {}", sum, num, sum * num);
                    return;
                }
            }
        }

        for index in winners.iter().rev() {
            boards.remove(*index);
        }
    }
}

fn part_one(drawn_numbers: &Vec<u32>, boards: &mut Vec<Vec<Vec<Cell>>>) {
    for (i, num) in drawn_numbers.iter().enumerate() {
        *boards = boards
            .into_iter()
            .map(|mut board| mark_number(&mut board, *num).to_vec())
            .collect();

        for (j, board) in boards.iter().enumerate() {
            if check_for_win(&board) {
                println!(
                    "pt 1: board {} won first in round {}, after drawing {}",
                    j + 1,
                    i + 1,
                    num
                );
                println!("winning score: {}", sum_unmarked(&board) * num);
                return;
            }
        }
    }
}

fn sum_unmarked(board: &Vec<Vec<Cell>>) -> u32 {
    board
        .iter()
        .map(|row| {
            row.iter()
                .filter(|cell| !cell.marked)
                .map(|cell| cell.value)
                .sum::<u32>()
        })
        .sum()
}

fn check_for_win(board: &Vec<Vec<Cell>>) -> bool {
    check_rows_for_win(&board) || check_columns_for_win(&board)
}

fn check_rows_for_win(board: &Vec<Vec<Cell>>) -> bool {
    for row in board.iter() {
        if row.iter().all(|cell| cell.marked) {
            return true;
        }
    }

    false
}

fn check_columns_for_win(board: &Vec<Vec<Cell>>) -> bool {
    for index in 0..board[0].len() {
        if board.iter().all(|row| row[index].marked) {
            return true;
        }
    }

    false
}

fn mark_number(board: &mut Vec<Vec<Cell>>, number: u32) -> &mut Vec<Vec<Cell>> {
    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            if cell.value == number {
                cell.marked = true;
            }
        }
    }

    board
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ value: {}, marked: {} }}", self.value, self.marked)
    }
}
