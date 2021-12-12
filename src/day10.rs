use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

#[allow(dead_code)]
fn exec1(filename: &str) -> i32 {
    let input = read_file(filename);
    let closing_to_open_char: HashMap<char, char> =
        HashMap::from([('}', '{'), ('>', '<'), (')', '('), (']', '[')]);
    let closing_chars: Vec<char> = Vec::from(['}', '>', ')', ']']);

    let char_to_value: HashMap<char, i32> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let mut sum = 0;
    for input_line in input {
        let mut queue: VecDeque<char> = VecDeque::new();

        for elem in input_line {
            if !closing_chars.contains(&elem) {
                queue.push_front(elem)
            } else {
                let required_closing = queue.get(0).unwrap();
                if required_closing == closing_to_open_char.get(&elem).unwrap() {
                    queue.pop_front();
                } else {
                    sum += char_to_value.get(&elem).unwrap();
                    break;
                }
            }
        }
    }

    return sum;
}

#[allow(dead_code)]
fn exec2(filename: &str) -> i64 {
    let input = read_file(filename);
    let closing_to_open_char: HashMap<char, char> =
        HashMap::from([('}', '{'), ('>', '<'), (')', '('), (']', '[')]);
    let closing_chars: Vec<char> = Vec::from(['}', '>', ')', ']']);
    let char_to_value: HashMap<char, i32> = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let mut sum_vector = Vec::new();

    for input_line in input {
        let mut queue: VecDeque<char> = VecDeque::new();

        let mut corrupted = false;
        for elem in &input_line {
            if !closing_chars.contains(&elem) {
                queue.push_front(*elem)
            } else {
                let required_closing = queue.get(0).unwrap();
                if required_closing == closing_to_open_char.get(&elem).unwrap() {
                    queue.pop_front();
                } else {
                    corrupted = true;
                    break;
                }
            }
        }
        if corrupted {
            continue;
        }

        let mut sum: i64 = 0;
        for value in &queue {
            sum = (sum * 5) + *char_to_value.get(&value).unwrap() as i64;
        }
        sum_vector.push(sum);
    }

    sum_vector.sort();
    return sum_vector[(sum_vector.len() - 1) / 2];
}

fn read_file(filename: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(filename).expect("error");
    return contents
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.trim().chars().collect())
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_exec1() {
        assert_eq!(exec1("input/day10/input.txt"), 364389);
    }

    #[test]
    fn day10_exec2() {
        assert_eq!(exec2("input/day10/test.txt"), 288957);
        assert_eq!(exec2("input/day10/input.txt"), 2870201088);
    }
}
