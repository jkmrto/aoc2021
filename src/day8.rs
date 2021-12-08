use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

//Optimization: Start from mean instead of from 0 ??

#[allow(dead_code)]
fn exec1() {
    //    let filename = "input/day8/test.txt";
    let filename = "input/day8/input.txt";
    let contents = fs::read_to_string(filename).expect("error");
    let entries: Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> = contents
        .trim()
        .split("\n")
        .map(|line| parse_line(line))
        .map(|x: Vec<Vec<Vec<char>>>| (x[0].clone(), x[1].clone()))
        .collect();

    let mut sum = 0;
    for entry in entries {
        let entry_count = entry
            .1
            .iter()
            .filter(|x| [2, 4, 3, 7].contains(&x.len()))
            .count();
        sum += entry_count;
    }

    println!("Reult: {}", sum)
}

#[allow(dead_code)]
fn exec2() {
    // let filename = "input/day8/test.txt";
    let filename = "input/day8/input.txt";
    let contents = fs::read_to_string(filename).expect("error");
    let entries: Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> = contents
        .trim()
        .split("\n")
        .map(|line| parse_line(line))
        .map(|x: Vec<Vec<Vec<char>>>| (x[0].clone(), x[1].clone()))
        .collect();

    let mut total_sum = 0;
    for mut entry in entries {
        let mut one = find_one(&entry);
        let mut four = find_four(&entry);
        let mut six = find_six(&entry);
        let mut seven = find_seven(&entry);
        let mut eight = find_eight(&entry);
        let mut nine = find_nine(&entry, &one, &four);
        let mut zero = find_zero(&entry, &one, &four);
        let mut five = find_five(&entry, &six);
        let mut three = find_three(&entry, &one);
        let mut two = find_two(&entry, &six, &one);

        zero.sort();
        one.sort();
        two.sort();
        three.sort();
        four.sort();
        five.sort();
        six.sort();
        seven.sort();
        eight.sort();
        nine.sort();

        let list = [
            (zero, 0),
            (one, 1),
            (two, 2),
            (three, 3),
            (four, 4),
            (five, 5),
            (six, 6),
            (seven, 7),
            (eight, 8),
            (nine, 9),
        ];

        let mut register: HashMap<Vec<char>, i32> = HashMap::new();
        for (number_ligths, value) in list {
            register.insert(number_ligths, value);
        }

        let entry_length = entry.1.clone().len() as u32;
        let mut sum = 0;
        for (pos, output_chars) in entry.1.iter_mut().enumerate() {
            output_chars.sort();

            let value = register.get(output_chars).unwrap();
            sum += value * i32::pow(10, entry_length - pos as u32 - 1);
        }
        total_sum += sum
    }

    println!("Result: {}", total_sum)
}

fn find_three(entry: &(Vec<Vec<char>>, Vec<Vec<char>>), one: &Vec<char>) -> Vec<char> {
    let set_one: HashSet<&char> = one.iter().collect();

    let digits_with_five: Vec<&Vec<char>> = entry.0.iter().filter(|x| x.len() == 5).collect();
    let list: Vec<&Vec<char>> = digits_with_five
        .into_iter()
        .filter(|digit| intersect(digit, &set_one).len() == 2)
        .collect();

    if list.len() != 1 {
        panic!()
    };

    return list[0].clone();
}

fn find_five(entry: &(Vec<Vec<char>>, Vec<Vec<char>>), six: &Vec<char>) -> Vec<char> {
    let set_six: HashSet<&char> = six.iter().collect();

    let digits_with_five: Vec<&Vec<char>> = entry.0.iter().filter(|x| x.len() == 5).collect();
    let list: Vec<&Vec<char>> = digits_with_five
        .into_iter()
        .filter(|digit| intersect(digit, &set_six).len() == 5)
        .collect();

    if list.len() != 1 {
        panic!()
    };

    return list[0].clone();
}

fn find_two(
    entry: &(Vec<Vec<char>>, Vec<Vec<char>>),
    six: &Vec<char>,
    one: &Vec<char>,
) -> Vec<char> {
    let set_one: HashSet<&char> = one.iter().collect();
    let set_six: HashSet<&char> = six.iter().collect();

    let digits_with_five: Vec<&Vec<char>> = entry.0.iter().filter(|x| x.len() == 5).collect();
    let list: Vec<&Vec<char>> = digits_with_five
        .into_iter()
        .filter(|digit| intersect(digit, &set_one).len() == 1)
        .filter(|digit| intersect(digit, &set_six).len() == 4)
        .collect();

    if list.len() != 1 {
        panic!()
    };

    return list[0].clone();
}

fn find_zero(
    entry: &(Vec<Vec<char>>, Vec<Vec<char>>),
    one: &Vec<char>,
    four: &Vec<char>,
) -> Vec<char> {
    let set_one: HashSet<&char> = one.iter().collect();
    let set_four: HashSet<&char> = four.iter().collect();

    let zero_six_or_nine: Vec<&Vec<char>> = entry.0.iter().filter(|x| x.len() == 6).collect();
    let list: Vec<&Vec<char>> = zero_six_or_nine
        .into_iter()
        .filter(|digit| intersect(digit, &set_one).len() == 2)
        .filter(|digit| intersect(digit, &set_four).len() == 3)
        .collect();

    if list.len() != 1 {
        panic!()
    };

    return list[0].clone();
}

fn find_one(entry: &(Vec<Vec<char>>, Vec<Vec<char>>)) -> Vec<char> {
    entry
        .0
        .clone()
        .into_iter()
        .filter(|x| x.len() == 2)
        .collect::<Vec<Vec<char>>>()[0]
        .clone()
}

fn find_four(entry: &(Vec<Vec<char>>, Vec<Vec<char>>)) -> Vec<char> {
    entry
        .0
        .clone()
        .into_iter()
        .filter(|x| x.len() == 4)
        .collect::<Vec<Vec<char>>>()[0]
        .clone()
}

fn find_seven(entry: &(Vec<Vec<char>>, Vec<Vec<char>>)) -> Vec<char> {
    entry
        .0
        .clone()
        .into_iter()
        .filter(|x| x.len() == 3)
        .collect::<Vec<Vec<char>>>()[0]
        .clone()
}

fn find_six(entry: &(Vec<Vec<char>>, Vec<Vec<char>>)) -> Vec<char> {
    let one = find_one(&entry);
    let set_one: HashSet<&char> = one.iter().collect();

    let zero_six_or_nine: Vec<&Vec<char>> = entry.0.iter().filter(|x| x.len() == 6).collect();
    let six_list: Vec<&Vec<char>> = zero_six_or_nine
        .into_iter()
        .filter(|digit| intersect(digit, &set_one).len() == 1)
        .collect();

    return six_list[0].clone();
}

fn find_eight(entry: &(Vec<Vec<char>>, Vec<Vec<char>>)) -> Vec<char> {
    entry
        .0
        .clone()
        .into_iter()
        .filter(|x| x.len() == 7)
        .collect::<Vec<Vec<char>>>()[0]
        .clone()
}

fn find_nine(
    entry: &(Vec<Vec<char>>, Vec<Vec<char>>),
    one: &Vec<char>,
    four: &Vec<char>,
) -> Vec<char> {
    let set_one: HashSet<&char> = one.iter().collect();
    let set_four: HashSet<&char> = four.iter().collect();

    let zero_six_or_nine: Vec<&Vec<char>> = entry.0.iter().filter(|x| x.len() == 6).collect();
    let six_list: Vec<&Vec<char>> = zero_six_or_nine
        .into_iter()
        .filter(|digit| intersect(digit, &set_one).len() == 2)
        .filter(|digit| intersect(digit, &set_four).len() == 4)
        .collect();

    return six_list[0].clone();
}

fn intersect(digit: &Vec<char>, set_one: &HashSet<&char>) -> Vec<char> {
    return digit
        .into_iter()
        .collect::<HashSet<&char>>()
        .intersection(&set_one)
        .map(|x| **x)
        .collect::<Vec<char>>();
}

fn parse_line(line: &str) -> Vec<Vec<Vec<char>>> {
    return line
        .split("|")
        .map(|entries: &str| {
            entries
                .trim()
                .split(" ")
                .map(|s| s.chars().collect())
                .collect()
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_exec1() {
        exec1();
    }

    #[test]
    fn day8_exec2() {
        exec2();
    }
}
