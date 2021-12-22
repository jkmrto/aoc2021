use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
fn exec(filename: &str, steps: i32) -> i64 {
    let contents = fs::read_to_string(filename).expect("error");
    let split_content: Vec<&str> = contents.trim().split("\n\n").collect();

    let mut reactions: HashMap<(char, char), char> = HashMap::new();
    let mut register: HashMap<(char, char), i64> = HashMap::new();

    // Start and End of sequence are not repeated
    let sequence: Vec<char> = split_content[0].chars().collect();
    let seq_start = sequence[0];
    let seq_end = sequence[sequence.len() - 1];

    let insertion_rules_raw: Vec<&str> = split_content[1].split("\n").collect();

    for rule_raw in insertion_rules_raw {
        let rule_raw_vec: Vec<&str> = rule_raw.trim().split(" -> ").collect();
        let rule_raw_input_vec: Vec<char> = rule_raw_vec[0].chars().collect();
        let rule_raw_output_vec: Vec<char> = rule_raw_vec[1].chars().collect();
        reactions.insert(
            (rule_raw_input_vec[0], rule_raw_input_vec[1]),
            rule_raw_output_vec[0],
        );
    }

    for i in 0..(sequence.len() - 1) {
        *register.entry((sequence[i], sequence[i + 1])).or_default() += 1;
    }

    for _i in 0..steps {
        let mut output_register: HashMap<(char, char), i64> = HashMap::new();
        for (c, v) in &register {
            let output = *reactions.get(&(c.0, c.1)).unwrap();
            *output_register.entry((c.0, output)).or_default() += v as &i64;
            *output_register.entry((output, c.1)).or_default() += v as &i64;
        }
        register = output_register.clone();
    }

    let mut output: HashMap<char, i64> = HashMap::new();
    for (c, v) in register {
        *output.entry(c.0).or_default() += v;
        *output.entry(c.1).or_default() += v;
    }

    // tricky, tricky
    *output.entry(seq_start).or_default() += 1;
    *output.entry(seq_end).or_default() += 1;

    let max = output.values().max().unwrap() / 2;
    let min = output.values().min().unwrap() / 2;

    return max - min;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_exec1() {
        assert_eq!(exec("input/day14/example.txt", 10), 1588);
        assert_eq!(exec("input/day14/input.txt", 10), 5656);
    }

    #[test]
    fn day14_exec2() {
        assert_eq!(exec("input/day14/example.txt", 40), 2188189693529);
        assert_eq!(exec("input/day14/input.txt", 40), 12271437788530);
    }
}
