use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[allow(dead_code)]
fn exec1(filename: &str) -> i32 {
    let costs = read_board(filename);
    return run(costs);
}

#[allow(dead_code)]
fn exec2(filename: &str) -> i32 {
    let costs = read_board(filename);

    let mut costs_5x_wider = Vec::new();
    for line in &costs {
        let mut line_updated = Vec::new();
        for index in 0..5 {
            for elem in line {
                let value = elem + index;
                let value = if value > 9 { value - 9 } else { value };
                line_updated.push(value)
            }
        }
        costs_5x_wider.push(line_updated);
    }

    let mut costs_5x_wider_5x_longer = Vec::new();
    for index in 0..5 {
        for line in &costs_5x_wider {
            let mut line_updated = Vec::new();
            for elem in line {
                let value = elem + index;
                let value = if value > 9 { value - 9 } else { value };
                line_updated.push(value)
            }
            costs_5x_wider_5x_longer.push(line_updated);
        }
    }

    return run(costs_5x_wider_5x_longer);
}

fn run(costs: Vec<Vec<i32>>) -> i32 {
    let mut paths: Vec<Vec<((i32, i32), i32)>> = [[((0, 0), 0)].to_vec()].to_vec();
    let mut old_destinies_cost: HashMap<(i32, i32), i32> = HashMap::new();
    old_destinies_cost.insert((0, 0), 0);

    let width = costs[0].len() as i32;
    let height = costs.len() as i32;

    for _step in 0..20000 {
        let mut new_paths = Vec::new();
        for path in paths {
            let mut temp_path = move_on_path(&path, &costs);
            new_paths.append(&mut temp_path);
        }
        paths = new_paths;

        paths = paths
            .into_iter()
            .filter(|path| {
                let last_pos = path[path.len() - 1];
                match old_destinies_cost.get(&last_pos.0) {
                    Some(old_path_cost) => return old_path_cost > &last_pos.1,
                    None => return true,
                };
            })
            .collect();

        let mut least_current_paths_cost: HashMap<(i32, i32), i32> = HashMap::new();
        for path in &paths {
            let last_pos = path[path.len() - 1];
            match least_current_paths_cost.get(&last_pos.0) {
                Some(old_path_cost) => {
                    if *old_path_cost > last_pos.1 {
                        least_current_paths_cost.insert(last_pos.0, last_pos.1);
                    }
                }
                None => {
                    least_current_paths_cost.insert(last_pos.0, last_pos.1);
                    ()
                }
            };
        }

        for path in &paths {
            let last_pos = path[path.len() - 1];
            old_destinies_cost.insert(last_pos.0, last_pos.1);
        }

        let mut paths_already_added = HashSet::new();
        let mut new_paths = Vec::new();
        for path in &paths {
            let last_pos = path[path.len() - 1];
            let least_current_cost = least_current_paths_cost.get(&last_pos.0).unwrap();
            if *least_current_cost == last_pos.1 && !paths_already_added.contains(&last_pos.0) {
                new_paths.push(path.clone());
                paths_already_added.insert(last_pos.0);
            };
        }

        paths = new_paths;

        if paths.len() == 1 {
            let last_pos = paths[0][paths[0].len() - 1];
            if last_pos.0 .0 == (height - 1) && last_pos.0 .1 == (width - 1) {
                break;
            }
        }
    }

    return paths[0][paths[0].len() - 1].1;
}

fn dump_paths(paths: &Vec<Vec<((i32, i32), i32)>>) {
    println!("Paths: ");
    for p in paths {
        println!(" -> {:?}", p);
    }
}

fn move_on_path(
    path: &Vec<((i32, i32), i32)>,
    costs: &Vec<Vec<i32>>,
) -> Vec<Vec<((i32, i32), i32)>> {
    let width = costs[0].len() as i32;
    let height = costs.len() as i32;

    let new_positions = get_new_positions(path[path.len() - 1].0, width, height);

    let mut new_paths = Vec::new();
    for new_pos in &new_positions {
        let mut path = path.clone();
        path.push((
            *new_pos,
            path[path.len() - 1].1 + costs[new_pos.0 as usize][new_pos.1 as usize],
        ));
        new_paths.push(path)
    }

    return new_paths;
}

fn get_new_positions(pos: (i32, i32), width: i32, height: i32) -> Vec<(i32, i32)> {
    let steps = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut new_positions = Vec::new();
    for (step_y, step_x) in steps {
        if pos.0 + step_y < 0 || pos.0 + step_y >= height {
            continue;
        }
        if pos.1 + step_x < 0 || pos.1 + step_x >= width {
            continue;
        }

        new_positions.push((pos.0 + step_y, pos.1 + step_x));
    }

    return new_positions;
}

fn read_board(filename: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(filename).expect("error");

    let lines: Vec<&str> = contents.trim().split("\n").collect();
    let mut costs: Vec<Vec<i32>> = Vec::new();
    for line in &lines {
        let parsed_line: Vec<i32> = line
            .split("")
            .filter(|c| *c != "")
            .map(|c| c.parse().unwrap())
            .collect();
        costs.push(parsed_line);
    }

    return costs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_exec1() {
        assert_eq!(exec1("input/day15/example.txt"), 40);
        assert_eq!(exec1("input/day15/input.txt"), 748);
    }

    #[test]
    fn day15_exec2() {
        assert_eq!(exec2("input/day15/example.txt"), 315);
        assert_eq!(exec2("input/day15/input.txt"), 3045);
    }
}
