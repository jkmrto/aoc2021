use std::fs;

#[allow(dead_code)]
fn exec1() {
    // let filename = "input/day2/test.txt";
    let filename = "input/day2/input.txt";
    let input_lines: Vec<String> = read_file(filename);

    let mut commands = Vec::new();
    for line in input_lines.iter() {
        if line != &"" {
            let parsed_line: Vec<&str> = line.split(" ").collect();
            let steps: i32 = parsed_line[1].parse().unwrap();
            let command: (i32, i32) = match parsed_line[0] {
                "forward" => (steps, 0),
                "up" => (0, -steps),
                "down" => (0, steps),
                _ => (0, 0),
            };
            commands.push(command)
        }
    }

    let mut pos: (i32, i32) = (0, 0);
    for (_pos, command) in commands.iter().enumerate() {
        pos = (pos.0 + command.0, pos.1 + command.1)
    }

    println!("Result: {}", pos.0 * pos.1);
}

#[allow(dead_code)]
fn exec2() {
    // let filename = "input/day2/test.txt";
    let filename = "input/day2/input.txt";
    let input_lines: Vec<String> = read_file(filename);

    let mut pos: (i32, i32, i32) = (0, 0, 0);
    for line in input_lines.iter() {
        if line != &"" {
            let parsed_line: Vec<&str> = line.split(" ").collect();
            let steps: i32 = parsed_line[1].parse().unwrap();

            // (x, y, aim)
            pos = match parsed_line[0] {
                "forward" => (pos.0 + steps, pos.1 + steps * pos.2, pos.2),
                "up" => (pos.0, pos.1, pos.2 - steps),
                "down" => (pos.0, pos.1, pos.2 + steps),
                _ => pos,
            };
        }
    }

    println!("Result: {}", pos.0 * pos.1);
}

#[allow(dead_code)]
fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("error");
    return contents.split("\n").map(&str::to_string).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_exec1() {
        exec1();
    }

    #[test]
    fn day2_exec2() {
        exec2();
    }
}
