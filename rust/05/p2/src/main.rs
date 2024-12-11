use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let mut rules: Vec<String> = Vec::new();
    let mut updates: Vec<String> = Vec::new();
    let mut updates_section = false; 

    for line in lines.flatten() {
        if line == "" {
            updates_section = true;
            continue;
        }

        if updates_section {
            updates.push(line);
        } else {
            rules.push(line);
        }
    }

    let mut total: u128 = 0;
    let dependency_graph = get_dependency_graph(rules);

    for update in &updates {
        let parts: Vec<u8> = update
            .split(",")
            .map(|x| x.parse::<u8>().expect("failed to parse update num"))
            .collect();

        let relative_dependency_graph = get_relative_dependency_graph(&parts, &dependency_graph);

        if !is_update_valid(&parts, &relative_dependency_graph) {
            let sorted_parts = topological_sort(&parts, &relative_dependency_graph);
            let sanity_check = is_update_valid(&sorted_parts, &relative_dependency_graph);
            assert!(sanity_check);
            total += sorted_parts[sorted_parts.len() / 2] as u128;
        }
    }

    println!("{}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_update_valid(parts: &[u8], dependency_graph: &HashMap<u8, HashSet<u8>>) -> bool {
    let index_map = parts.iter().enumerate().map(|(i, &x)| (x, i)).collect::<HashMap<_, _>>();

    fn dfs(
        root: usize,
        part: u8,
        index_map: &HashMap<u8, usize>,
        dependency_graph: &HashMap<u8, HashSet<u8>>,
        visited: &mut HashSet<u8>,
    ) -> bool {
        visited.insert(part);

        if let Some(children) = dependency_graph.get(&part) {
            for &child in children {
                if visited.contains(&child) {
                    continue;
                }

                if let Some(&child_index) = index_map.get(&child) {
                    if child_index < root {
                        return false;
                    }

                    if dfs(root, child, index_map, dependency_graph, visited) == false {
                        return false;
                    }
                }
            }
        }

        return true;
    }

    for (i, &part) in parts.iter().enumerate() {
        let mut visited: HashSet<u8> = HashSet::new();

        if dfs(i, part, &index_map, &dependency_graph, &mut visited) == false {
            return false;
        }
    }
    
    return true;
}

fn get_dependency_graph(rules: Vec<String>) -> HashMap<u8, HashSet<u8>> {
    let mut dependency_graph: HashMap<u8, HashSet<u8>> = HashMap::new();

    for rule in rules {
        let parts: Vec<&str> = rule.split("|").collect();
        assert!(parts.len() == 2);

        let num1: u8 = parts[0].parse().expect("failed to parse num1");
        let num2: u8 = parts[1].parse().expect("failed to parse num2");

        dependency_graph
            .entry(num1)
            .or_insert(HashSet::new())
            .insert(num2);
    }

    dependency_graph
}

fn topological_sort(parts: &[u8], dependency_graph: &HashMap<u8, HashSet<u8>>) -> Vec<u8> {
    let mut sorted_parts = Vec::new();
    let mut visited = HashSet::new();

    fn dfs(
        node: u8,
        dependency_graph: &HashMap<u8, HashSet<u8>>,
        visited: &mut HashSet<u8>,
        sorted_parts: &mut Vec<u8>,
    ) {
        if visited.contains(&node) {
            return;
        }

        visited.insert(node);

        if let Some(children) = dependency_graph.get(&node) {
            for &child in children {
                dfs(child, dependency_graph, visited, sorted_parts);
            }
        }

        sorted_parts.push(node);
    }

    for &part in parts {
        if !visited.contains(&part) {
            dfs(part, dependency_graph, &mut visited, &mut sorted_parts);
        }
    }

    sorted_parts.reverse();
    sorted_parts
}

fn get_relative_dependency_graph(parts: &[u8], dependency_graph: &HashMap<u8, HashSet<u8>>) -> HashMap<u8, HashSet<u8>> {
    let parts_set = parts.iter().cloned().collect::<HashSet<_>>();
    let mut relative_dependency_graph: HashMap<u8, HashSet<u8>> = HashMap::new();

    for &part in parts {
        if let Some(dependencies) = dependency_graph.get(&part) {
            let filtered_dependencies: HashSet<u8> = dependencies
                .iter()
                .filter(|&&x| parts_set.contains(&x))
                .cloned()
                .collect();

            relative_dependency_graph.insert(part, filtered_dependencies);
        }
    }

    relative_dependency_graph
}