use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
pub fn exec1() {
    // let filename: &str = "input/day5/test.txt";
    let filename: &str = "input/day5/input.txt";
    let contents = fs::read_to_string(filename).expect("error");
    let vec = parse_winds_vectors(contents);
    let filter_winds: Vec<((i32, i32), (i32, i32))> = vec
        .into_iter()
        .filter(|x| x.0 .0 == x.1 .0 || x.0 .1 == x.1 .1)
        .collect();

    let wind_points = apply_winds_vectors(&filter_winds);
    println!(
        "Solution: {}",
        wind_points.values().filter(|v| **v > 1).count()
    )
}

#[allow(dead_code)]
pub fn exec2() {
    // let filename: &str = "input/day5/test.txt";
    let filename: &str = "input/day5/input.txt";
    let contents = fs::read_to_string(filename).expect("error");
    let vec = parse_winds_vectors(contents);
    let filter_winds: Vec<((i32, i32), (i32, i32))> = vec
        .into_iter()
        .filter(|x| {
            (x.0 .0 == x.1 .0)
                || (x.0 .1 == x.1 .1)
                || (((x.0 .0 - x.1 .0) / (x.0 .1 - x.1 .1)).abs() == 1)
        })
        .collect();

    let wind_points = apply_winds_vectors(&filter_winds);
    println!(
        "Solution: {}",
        wind_points.values().filter(|v| **v > 1).count()
    );
}

fn apply_winds_vectors(winds_vectors: &Vec<((i32, i32), (i32, i32))>) -> HashMap<(i32, i32), i32> {
    let mut wind_points: HashMap<(i32, i32), i32> = HashMap::new();
    for wind_vector in winds_vectors {
        let winds = find_wind_points(*wind_vector);
        for wind_point in winds {
            *wind_points.entry(wind_point).or_default() += 1;
        }
    }

    return wind_points;
}

fn parse_winds_vectors(file_string: String) -> Vec<((i32, i32), (i32, i32))> {
    return file_string
        .trim()
        .split("\n")
        .map(|x| parse_line(x.to_string()))
        .collect();
}

fn find_wind_points(wind_vector: ((i32, i32), (i32, i32))) -> Vec<(i32, i32)> {
    let module = if wind_vector.1 .0 - wind_vector.0 .0 == 0 {
        (wind_vector.1 .1 - wind_vector.0 .1).abs()
    } else {
        (wind_vector.1 .0 - wind_vector.0 .0).abs()
    };

    let direction = (
        (wind_vector.1 .0 - wind_vector.0 .0) / module,
        (wind_vector.1 .1 - wind_vector.0 .1) / module,
    );

    let mut winds = Vec::new();
    let mut current_pos = wind_vector.0;
    while current_pos != wind_vector.1 {
        winds.push(current_pos);
        current_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
    }
    winds.push(current_pos);

    return winds;
}

fn parse_line(line: String) -> ((i32, i32), (i32, i32)) {
    let parsed_line: Vec<(i32, i32)> = line
        .split(" -> ")
        .map(|x| x.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .map(|x: Vec<i32>| (x[0], x[1]))
        .collect();

    return (parsed_line[0], parsed_line[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_exec1() {
        exec1();
    }

    #[test]
    fn day5_exec2() {
        exec2();
    }
}

// Debug utils

#[allow(dead_code)]
fn dump_winds_vectors(winds_vectors: &Vec<((i32, i32), (i32, i32))>) {
    for ((x1, y1), (x2, y2)) in winds_vectors {
        println!("{}, {} -> {}, {}", x1, y1, x2, y2)
    }
}

#[allow(dead_code)]
fn dump_points(wind_vector: ((i32, i32), (i32, i32)), winds: &Vec<(i32, i32)>) {
    print!(
        "found points on vector: (({}, {}), ({}, {})) => ",
        wind_vector.0 .0, wind_vector.0 .1, wind_vector.1 .0, wind_vector.1 .1
    );
    for (x, y) in winds {
        print!("({}, {}) ", x, y);
    }
    println!("");
}
