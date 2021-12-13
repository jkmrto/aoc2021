use std::fs;

#[allow(dead_code)]
fn exec1(filename: &str) -> i32 {
    let mut input = read_file(filename);

    let mut sum = 0;
    for _x in 0..100 {
        increasse_energy(&mut input);
        sum += apply_step(&mut input);
    }

    return sum;
}

#[allow(dead_code)]
fn exec2(filename: &str) -> i32 {
    let mut input = read_file(filename);

    let mut turn_all_flashes = 0;
    for turn in 1..100000 {
        increasse_energy(&mut input);
        apply_step(&mut input);

        let mut all_zeros = true;
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if input[y][x] != 0 {
                    all_zeros = false;
                    break;
                }
            }
            if !all_zeros {
                break;
            }
        }

        if all_zeros {
            turn_all_flashes = turn;
            break;
        }
    }

    return turn_all_flashes;
}

fn read_file(filename: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(filename).expect("error");
    return contents
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
}

fn increasse_energy(lines: &mut Vec<Vec<i32>>) {
    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            lines[y][x] += 1;
        }
    }
}

fn apply_step(lines: &mut Vec<Vec<i32>>) -> i32 {
    let mut any_flash = true;
    let mut n_flashes = 0;

    while any_flash {
        any_flash = false;

        for y in 0..lines.len() {
            for x in 0..lines[0].len() {
                if lines[y][x] > 9 {
                    lines[y][x] = 0;
                    n_flashes += 1;
                    for y_1 in [-1, 0, 1] {
                        for x_1 in [-1, 0, 1] {
                            if !(y_1 == 0 && x_1 == 0) {
                                let (actual_y, actual_x) =
                                    (y as i32 + y_1 as i32, x as i32 + x_1 as i32);

                                if (actual_y >= 0)
                                    && (actual_x >= 0)
                                    && (actual_y < lines.len() as i32)
                                    && (actual_x < lines[0].len() as i32)
                                    && (lines[actual_y as usize][actual_x as usize] != 0)
                                {
                                    lines[actual_y as usize][actual_x as usize] += 1;
                                    any_flash = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return n_flashes;
}

fn dump(lines: &Vec<Vec<i32>>) {
    println!();
    for line in lines {
        for pos in line {
            print!("{},", pos);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_exec1() {
        assert_eq!(exec1("input/day11/input.txt"), 1562);
    }

    #[test]
    fn day11_exec2() {
        assert_eq!(exec2("input/day11/test.txt"), 195);
        assert_eq!(exec2("input/day11/input.txt"), 268);
    }
}
