use std::fs;

// Function that returns a boolean value
fn exec1() {
    let filename = "input/day1/input.txt";
    let contents = fs::read_to_string(filename).expect("error");
    let split = contents.split("\n");
    let vec: Vec<&str> = split.collect();

    let mut acc: u32 = 0;

    let mut xs = Vec::new();
    for line in vec.iter() {
        if line != &"" {
            let my_int: i32 = line.parse().unwrap();
            xs.push(my_int);
        }
    }

    for (pos, _e) in xs[1..xs.len()].iter().enumerate() {
        if xs[pos + 1] > xs[pos] {
            acc = acc + 1;
        }
    }

    println!("acc result: {}", acc);
}

fn exec2() {
    // let filename = "input/day1/test.txt";
    let filename = "input/day1/input.txt";
    let contents = fs::read_to_string(filename).expect("error");
    let split = contents.split("\n");
    let vec: Vec<&str> = split.collect();

    let mut acc: u32 = 0;

    let mut xs = Vec::new();
    for line in vec.iter() {
        if line != &"" {
            let my_int: i32 = line.parse().unwrap();
            xs.push(my_int);
        }
    }

    let mut previous_window: i32 = 99999;
    for (pos, _e) in xs[1..xs.len() - 1].iter().enumerate() {
        let current_window = xs[pos] + xs[pos + 1] + xs[pos + 2];
        if previous_window < current_window {
            acc = acc + 1;
        }
        previous_window = current_window
    }

    println!(">>>> Exercise 2 result: {} <<<<<", acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_exec1() {
        exec1();
    }

    #[test]
    fn day1_exec2() {
        exec2();
    }
}
