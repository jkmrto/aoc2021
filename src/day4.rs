use std::fs;

#[allow(dead_code)]
fn exec1() {
    let filename: &str = "input/day4/input.txt";
    //let filename: &str = "input/day4/test.txt";
    let (sequence, mut boards) = read_file(filename);

    let mut last_sequence_number = 0;
    let mut found_row = false;
    let mut board_sum = 0;
    for number in &sequence {
        check_number_on_board(&mut boards, *number);

        for board in boards.iter() {
            for board_line in board.iter() {
                let mut is_row_fullfil = true;
                for board_box in board_line.iter() {
                    if board_box.1 == false {
                        is_row_fullfil = false;
                        break;
                    }
                }

                if is_row_fullfil {
                    found_row = true;
                }
            }

            if found_row {
                board_sum = sum_unchecked_values(board);
                break;
            }
        }

        if found_row {
            last_sequence_number = *number;
            break;
        }
    }

    println!("Exec1 solution: {}", board_sum * last_sequence_number);
}

#[allow(dead_code)]
fn exec2() {
    let filename: &str = "input/day4/input.txt";
    // let filename: &str = "input/day4/test.txt";
    let (sequence, mut boards) = read_file(filename);

    let mut board_sum = 0;
    let mut last_sequence_number = 0;
    for number in &sequence {
        let mut to_remove = Vec::new();
        for (board_i, board) in boards.iter().enumerate() {
            if any_row_or_column_fullfil(&board) {
                if boards.len() == 1 {
                    board_sum = sum_unchecked_values(&board);
                }
                to_remove.push(board_i);
            }
        }

        for (pos, to_remoe_index) in to_remove.iter().enumerate() {
            boards.remove(to_remoe_index - pos);
        }

        if boards.len() == 0 {
            last_sequence_number = *number;
            break;
        }
    }

    println!("Exec2 solution: {}", board_sum);
    println!("Exec2 solution: {}", last_sequence_number);
    println!("Exec2 solution: {}", board_sum * last_sequence_number);
}

fn any_row_or_column_fullfil(board: &Vec<Vec<(i32, bool)>>) -> bool {
    // println!("Checking fullfilled rows");
    for board_line in board.iter() {
        let mut is_row_fullfil = true;
        for board_box in board_line.iter() {
            if board_box.1 == false {
                is_row_fullfil = false;
                break;
            }
        }

        if is_row_fullfil {
            return true;
        }
    }

    //  println!("Checking fullfilled colums");
    for j in 0..board[0].len() {
        let mut is_column_fullfil = true;
        for i in 0..board.len() {
            if board[i][j].1 == false {
                is_column_fullfil = false;
                break;
            }
        }

        if is_column_fullfil {
            return true;
        }
    }

    return false;
}

fn sum_unchecked_values(board: &Vec<Vec<(i32, bool)>>) -> i32 {
    let mut board_sum = 0;
    for board_line in board.iter() {
        for board_box in board_line.iter() {
            if board_box.1 == false {
                board_sum = board_sum + board_box.0;
            }
        }
    }
    return board_sum;
}

fn check_number_on_board(boards: &mut Vec<Vec<Vec<(i32, bool)>>>, number: i32) {
    for board in &mut boards.iter_mut() {
        for board_line in &mut board.iter_mut() {
            for board_box in &mut board_line.iter_mut() {
                if board_box.0 == number {
                    *board_box = (board_box.0, true)
                }
            }
        }
    }
}

fn read_file(filename: &str) -> (Vec<i32>, Vec<Vec<Vec<(i32, bool)>>>) {
    let contents = fs::read_to_string(filename).expect("error");
    let vec: Vec<String> = contents.split("\n\n").map(&str::to_string).collect();

    let sequence = build_sequence(&vec[0]);
    let boards = build_boards(&vec[1..vec.len()]);

    return (sequence, boards);
}

fn build_sequence(raw_sequence: &String) -> Vec<i32> {
    return raw_sequence
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn build_boards(raw_boards: &[String]) -> Vec<Vec<Vec<(i32, bool)>>> {
    let mut boards: Vec<Vec<Vec<(i32, bool)>>> = Vec::new();
    for raw_board in raw_boards {
        boards.push(
            raw_board
                .split("\n")
                .map(|board_line| parse_board_line(board_line.to_string()))
                .filter(|board_line| board_line.len() != 0)
                .collect(),
        )
    }

    return boards;
}

fn parse_board_line(board_line: String) -> Vec<(i32, bool)> {
    return board_line
        .split(" ")
        .filter(|x| x != &"")
        //.for_each(|x| parse_board_line(x.to_string()));
        .map(|x| (x.parse::<i32>().unwrap(), false))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_exec1() {
        exec1();
    }

    #[test]
    fn day4_exec2() {
        exec2();
    }
}

// Debug utils
#[allow(dead_code)]
fn dump_boards(boards: &Vec<Vec<Vec<(i32, bool)>>>) {
    for board in boards {
        println!("board");
        for line in board {
            dump_board_line(line);
            println!("");
        }
    }
}

#[allow(dead_code)]
fn dump_board_line(board_line: &Vec<(i32, bool)>) {
    for board_box in board_line.iter() {
        if board_box.1 {
            print!("{}(X) ", board_box.0);
        } else {
            print!("{}( ) ", board_box.0);
        }
    }
}
