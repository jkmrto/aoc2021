use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
fn exec1(filename: &str) -> i32 {
    let input = read_input(filename);
    let registry = build_registry(input);

    let mut paths: Vec<Vec<String>> = Vec::from([Vec::from(["start".to_string()])]);
    let mut all_path_at_end = false;
    while !all_path_at_end {
        paths = apply_step_exec1(&paths, &registry);
        all_path_at_end = paths
            .iter()
            .all(|path| path[path.len() - 1] == "end".to_string())
    }

    return paths.len() as i32;
}

#[allow(dead_code)]
fn exec2(filename: &str) -> i32 {
    let input = read_input(filename);
    let registry = build_registry(input);

    let mut paths: Vec<Vec<String>> = Vec::from([Vec::from(["start".to_string()])]);
    let mut all_path_at_end = false;
    while !all_path_at_end {
        paths = apply_step_exec2(&paths, &registry);
        all_path_at_end = paths
            .iter()
            .all(|path| path[path.len() - 1] == "end".to_string())
    }

    return paths.len() as i32;
}

fn read_input(filename: &str) -> Vec<(String, String)> {
    let contents = fs::read_to_string(filename).expect("error");

    let input: Vec<(String, String)> = contents
        .trim()
        .split("\n")
        .map(|line| {
            let line_split: Vec<&str> = line.split("-").collect();
            (line_split[0].to_string(), line_split[1].to_string())
        })
        .collect();

    return input;
}

fn build_registry(input: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut registry: HashMap<String, Vec<String>> = HashMap::new();
    for (origin, destiny) in input {
        if destiny != "start".to_string() {
            match registry.get_mut(&origin) {
                Some(destinies) => destinies.push(destiny.clone()),
                None => {
                    registry.insert(origin.clone(), Vec::from([destiny.clone()]));
                    ();
                }
            }
        }
        if destiny == "end".to_string() || origin == "start".to_string() {
            continue;
        }
        match registry.get_mut(&destiny) {
            Some(origins) => origins.push(origin),
            None => {
                registry.insert(destiny, Vec::from([origin]));
                ();
            }
        }
    }

    return registry;
}

fn apply_step_exec1(
    paths: &Vec<Vec<String>>,
    registry: &HashMap<String, Vec<String>>,
) -> Vec<Vec<String>> {
    let mut new_paths: Vec<Vec<String>> = Vec::new();
    for path in paths {
        if path[path.len() - 1] == "end".to_string() {
            new_paths.push(path.clone());
            continue;
        }

        let last_node_path = &path[path.len() - 1];
        let possible_next_jumps = registry.get(last_node_path).unwrap();

        let remaining_jumps: Vec<&String> = possible_next_jumps
            .into_iter()
            .filter(|&x| !(*x == x.to_lowercase() && path.contains(x)))
            .collect();

        for node_to_move in remaining_jumps {
            let mut new_path = path.clone();
            new_path.push(node_to_move.to_string());
            new_paths.push(new_path);
        }
    }

    return new_paths;
}

fn count_small_caves_ocurrences(path: &Vec<String>) -> HashMap<String, i32> {
    let small_caves_path: Vec<&String> = path
        .iter()
        .filter(|&pos| *pos == pos.to_lowercase())
        .collect();
    let mut small_caves_occurrences: HashMap<String, i32> = HashMap::new();
    for small_cave in small_caves_path {
        *small_caves_occurrences
            .entry(small_cave.clone())
            .or_default() += 1;
    }

    return small_caves_occurrences;
}

fn apply_step_exec2(
    paths: &Vec<Vec<String>>,
    registry: &HashMap<String, Vec<String>>,
) -> Vec<Vec<String>> {
    let mut new_paths: Vec<Vec<String>> = Vec::new();
    for path in paths {
        if path[path.len() - 1] == "end".to_string() {
            new_paths.push(path.clone());
            continue;
        }

        let small_caves_occurrences = count_small_caves_ocurrences(path);
        let mut one_small_caved_already_visited_twice = false;
        for (_small_cave, number) in small_caves_occurrences.iter() {
            if *number == 2 {
                one_small_caved_already_visited_twice = true;
                break;
            }
        }

        let last_node_path = &path[path.len() - 1];
        let possible_next_jumps = registry.get(last_node_path).unwrap();

        let remaining_jumps: Vec<&String> = possible_next_jumps
            .into_iter()
            .filter(|&x| {
                *x != x.to_lowercase()
                    || !one_small_caved_already_visited_twice
                    || !path.contains(x)
            })
            .collect();

        for node_to_move in remaining_jumps {
            let mut new_path = path.clone();
            new_path.push(node_to_move.to_string());
            new_paths.push(new_path);
        }
    }

    return new_paths;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_exec1() {
        assert_eq!(exec1("input/day12/example1.txt"), 10);
        assert_eq!(exec1("input/day12/example2.txt"), 19);
        assert_eq!(exec1("input/day12/example3.txt"), 226);
        assert_eq!(exec1("input/day12/input.txt"), 226);
    }

    #[test]
    fn day12_exec2() {
        assert_eq!(exec2("input/day12/example1.txt"), 36);
        assert_eq!(exec2("input/day12/example2.txt"), 103);
        assert_eq!(exec2("input/day12/example3.txt"), 3509);
        assert_eq!(exec2("input/day12/input.txt"), 84271);
    }
}
