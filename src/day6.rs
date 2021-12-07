use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
fn exec1() {
    // let filename: &str = "input/day6/test.txt";
    let filename: &str = "input/day6/input.txt";

    let contents = fs::read_to_string(filename).expect("error");
    let sequence: Vec<i32> = contents
        .trim()
        .split(",")
        .map(&str::to_string)
        .map(|x: String| x.parse().unwrap())
        .collect();

    let mut registry: HashMap<i32, i64> = HashMap::new();

    for lanterfish in sequence {
        *registry.entry(lanterfish).or_default() += 1;
    }

    println!("My registry: {:?}", registry);

    for day in 0..257 {
        let fish_to_reproduce = day - 1;
        if registry.get(&fish_to_reproduce).is_some() {
            let amount = *registry.get(&fish_to_reproduce).unwrap();
            *registry.entry(day + 8).or_default() += amount;
        }

        let fish_to_reset = day - 2;
        if registry.get(&fish_to_reset).is_some() {
            let amount = *registry.get(&fish_to_reset).unwrap();
            *registry.entry(day + 5).or_default() += amount;
        }

        registry.remove(&fish_to_reset);

        let mut sum = 0;
        for value in registry.values() {
            sum += value;
        }
        println!("Day {}, Acccount: {}, register. {:?}", day, sum, registry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_exec1() {
        exec1();
    }
}
