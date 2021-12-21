use std::collections::HashSet;
use std::fs;

fn read_file(filename: &str) -> (Vec<Vec<String>>, HashSet<(i32, i32)>, i32, i32) {
    let contents = fs::read_to_string(filename).expect("error");
    let split_content: Vec<&str> = contents.trim().split("\n\n").collect();
    let dots_positions: Vec<&str> = split_content[0].trim().split("\n").collect();

    let folds: Vec<Vec<String>> = split_content[1]
        .split("\n")
        .map(|x| {
            x.split(" ").collect::<Vec<&str>>()[2]
                .split("=")
                .map(|x| x.to_string())
                .collect()
        })
        .collect();

    let mut positions: HashSet<(i32, i32)> = HashSet::new();

    let mut max_x = 0;
    let mut max_y = 0;
    for raw_dot in dots_positions {
        let dot: Vec<i32> = raw_dot
            .trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        max_x = if dot[0] > max_x { dot[0] } else { max_x };
        max_y = if dot[1] > max_y { dot[1] } else { max_y };

        positions.insert((dot[0], dot[1]));
    }
    max_x += 1;
    max_y += 1;

    return (folds, positions, max_x, max_y);
}

#[allow(dead_code)]
fn exec1(filename: &str) -> i32 {
    let (folds, mut positions, _max_x, _max_y) = read_file(filename);

    let fold = &folds[0];

    if fold[0] == "x" {
        let fold_x = fold[1].parse().unwrap();
        positions = horizontal_fold(positions, fold_x);
    }
    if fold[0] == "y" {
        let fold_y = fold[1].parse().unwrap();
        positions = vertical_fold(positions, fold_y);
    }

    return positions.len() as i32;
}

#[allow(dead_code)]
fn exec2(filename: &str) -> i32 {
    let (folds, mut positions, mut max_x, mut max_y) = read_file(filename);

    for fold in folds {
        if fold[0] == "x" {
            let fold_x = fold[1].parse().unwrap();
            positions = horizontal_fold(positions, fold_x);
            max_x = max_x - fold_x - 1;
        }
        if fold[0] == "y" {
            let fold_y = fold[1].parse().unwrap();
            positions = vertical_fold(positions, fold_y);
            max_y = max_y - (max_y - fold_y);
        }
    }

    dump(&positions, max_x, max_y);
    return positions.len() as i32;
}

fn horizontal_fold(positions: HashSet<(i32, i32)>, fold_x: i32) -> HashSet<(i32, i32)> {
    let mut new_pos: HashSet<(i32, i32)> = HashSet::new();
    for dot in positions {
        if dot.0 == fold_x {
            continue;
        }
        if dot.0 > fold_x {
            new_pos.insert((fold_x - (dot.0 - fold_x), dot.1));
        }
        if dot.0 < fold_x {
            new_pos.insert(dot);
        }
    }

    return new_pos;
}

fn vertical_fold(positions: HashSet<(i32, i32)>, fold_y: i32) -> HashSet<(i32, i32)> {
    let mut new_pos: HashSet<(i32, i32)> = HashSet::new();
    for dot in positions {
        if dot.1 == fold_y {
            continue;
        }
        if dot.1 > fold_y {
            new_pos.insert((dot.0, fold_y - (dot.1 - fold_y)));
        }
        if dot.1 < fold_y {
            new_pos.insert(dot);
        }
    }

    return new_pos;
}

fn dump(positions: &HashSet<(i32, i32)>, max_x: i32, max_y: i32) {
    for j in 0..max_y {
        for i in 0..max_x {
            if positions.contains(&(i, j)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_exec1() {
        assert_eq!(exec1("input/day13/example.txt"), 10);
        assert_eq!(exec1("input/day13/input.txt"), 10);
    }

    #[test]
    fn day13_exec2() {
        assert_eq!(exec2("input/day13/example.txt"), 16);
        assert_eq!(exec2("input/day13/input.txt"), 89);
    }
}
