use std::fs;

//Optimization: Start from mean instead of from 0 ??

#[allow(dead_code)]
fn exec1() {
    // let filename = "input/day7/test.txt";
    let filename = "input/day7/input.txt";
    let sequence: Vec<i32> = read_sequence(filename);

    let mut least_fuel: i32 = sequence.iter().sum();
    let max = *sequence.iter().max().unwrap();

    for pos in 1..max {
        let current_fuel: i32 = sequence.iter().map(|x: &i32| (pos - *x).abs()).sum();
        println!("Fuel required -> pos: {}, fuel: {}", pos, current_fuel);
        if current_fuel < least_fuel {
            least_fuel = current_fuel;
        }
    }

    println!("Exec1 Solution: {}", least_fuel);
}

#[allow(dead_code)]
fn exec2() {
    // let filename = "input/day7/test.txt";
    let filename = "input/day7/input.txt";
    let sequence: Vec<i32> = read_sequence(filename);

    let mut least_fuel: i32 = sequence.iter().map(|x: &i32| progressive_fuel(*x, 0)).sum();
    let max = *sequence.iter().max().unwrap();

    println!("Fuel required -> pos: {}, fuel: {}", 0, least_fuel);
    for pos in 1..max {
        let current_fuel: i32 = sequence
            .iter()
            .map(|x: &i32| progressive_fuel(*x, pos))
            .sum();
        if current_fuel < least_fuel {
            least_fuel = current_fuel;
        }
    }

    println!("Exec1 Solution: {}", least_fuel);
}

fn progressive_fuel(origin_pos: i32, destiny_pos: i32) -> i32 {
    let distance = (1..(origin_pos - destiny_pos).abs() + 1).sum();
    return distance;
}

fn read_sequence(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("error");
    return contents
        .trim()
        .split(",")
        .map(&str::to_string)
        .map(|x: String| x.parse().unwrap())
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_exec1() {
        exec1();
    }

    #[test]
    fn day7_exec2() {
        exec2();
    }
}
