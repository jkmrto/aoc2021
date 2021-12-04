use std::fs;

#[allow(dead_code)]
fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("error");
    return contents.split("\n").map(&str::to_string).collect();
}

#[allow(dead_code)]
fn exec1() {
    //    let filename: &str = "input/day3/test.txt";
    let filename: &str = "input/day3/input.txt";
    let input_lines = read_file(filename);
    let seq = parse_lines_as_chars(input_lines);

    let mut result = 0;
    for j in 0..seq[0].len() {
        if find_more_common(&seq, j) == "1" {
            result = result + i32::pow(2, (seq[0].len() - (j + 1)).try_into().unwrap());
        }
    }

    let complementary = i32::pow(2, seq[0].len().try_into().unwrap()) - result - 1;

    println!("result: >>>> {} <<<<", result * complementary)
}

#[allow(dead_code)]
fn exec2() {
    //let filename: &str = "input/day3/test.txt";
    let filename: &str = "input/day3/input.txt";
    let mut seq = parse_lines_as_chars(read_file(filename));
    let seq2 = parse_lines_as_chars(read_file(filename));

    let mut result_seq_1 = Vec::new();
    for j in 0..seq[0].len() {
        let more_common = find_more_common(&seq, j);
        seq = filter(&seq, j, more_common);
        if seq.len() == 1 {
            result_seq_1 = seq[0].clone();
            break;
        }
    }

    seq = seq2;
    let mut result_seq_2 = Vec::new();
    for j in 0..seq[0].len() {
        let more_common = find_more_common(&seq, j);
        let least_common = (if more_common == "1" { "0" } else { "1" }).to_string();
        seq = filter(&seq, j, least_common);
        if seq.len() == 1 {
            result_seq_2 = seq[0].clone();
            break;
        }
    }

    println!(
        "exec 2 result: {}",
        build_decimal(&result_seq_1) * build_decimal(&result_seq_2)
    )
}

fn build_decimal(seq: &Vec<char>) -> i32 {
    let mut sum = 0;
    for i in 0..seq.len() {
        if seq[i].to_string() == "1" {
            sum = sum + i32::pow(2, (seq.len() - (i + 1)).try_into().unwrap());
        }
    }
    return sum;
}

#[allow(dead_code)]
fn dump_filter_seq(filtered_seq: &Vec<Vec<char>>) {
    for one_seq in filtered_seq {
        for elem in one_seq {
            print!("{}", elem)
        }
        println!("");
    }
}

fn filter(seq: &Vec<Vec<char>>, pos: usize, looked_bit: String) -> Vec<Vec<char>> {
    let mut filtered_seq = Vec::new();
    for i in 0..seq.len() {
        if &seq[i][pos].to_string()[..] == looked_bit {
            filtered_seq.push(seq[i].clone())
        }
    }
    return filtered_seq;
}

fn find_more_common(seq: &Vec<Vec<char>>, j: usize) -> String {
    let mut ones = 0;
    for i in 0..seq.len() {
        if &seq[i][j].to_string()[..] == "1" {
            ones = ones + 1
        }
    }

    let half_length = seq.len() as f64 / 2.0;
    let value = if (ones as f64) > half_length {
        "1".to_string()
    } else if (ones as f64) < half_length {
        "0".to_string()
    } else {
        "1".to_string()
    };

    return value;
}

fn parse_lines_as_chars(input_lines: Vec<String>) -> Vec<Vec<char>> {
    let mut seq = Vec::new();
    for line in input_lines.iter() {
        if line == &"" {
            continue;
        }

        let res: Vec<char> = line.chars().collect();
        seq.push(res)
    }
    return seq;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_exec1() {
        exec1();
    }

    #[test]
    fn day3_exec2() {
        exec2();
    }
}
