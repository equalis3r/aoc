use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("./assets/day12.txt");
type Small = bool;
type Graph<'a> = HashMap<&'a str, Vec<(&'a str, Small)>>;

pub fn solve() -> String {
    let input = parse(INPUT);
    let (no_small_paths, small_paths) = bfs(&input);
    format!(
        "  Part 1: {}\n  Part 2: {}",
        no_small_paths,
        no_small_paths + small_paths
    )
}

pub fn bfs(graph: &Graph) -> (u32, u32) {
    let mut small_paths: u32 = 0;
    let mut no_small_paths: u32 = 0;
    let mut to_visit = Vec::from([(("end", true), HashSet::new(), true)]);
    while let Some(((node, small), mut visited, mut twice)) = to_visit.pop() {
        if node == "start" {
            if twice {
                no_small_paths += 1;
            } else {
                small_paths += 1;
            }
        } else {
            if small {
                twice = twice && !visited.contains(node);
                visited.insert(node);
            }
            for (neighbor, neighbor_small) in &graph[&node] {
                if twice || !visited.contains(neighbor) {
                    to_visit.push(((neighbor, *neighbor_small), visited.clone(), twice));
                }
            }
        }
    }
    (no_small_paths, small_paths)
}

fn parse(input: &str) -> Graph<'_> {
    let mut graph = Graph::new();
    for line in input.lines() {
        let (n1, n2) = line.split_once('-').unwrap();
        if n1.ne("start") && n2.ne("end") {
            graph
                .entry(n1)
                .or_default()
                .push((n2, n2.to_lowercase() == n2));
        }
        if n2.ne("start") && n1.ne("end") {
            graph
                .entry(n2)
                .or_default()
                .push((n1, n1.to_lowercase() == n1));
        }
    }
    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_part1() {
        let test_input = parse(TEST_INPUT);
        let (no_small_paths, _) = bfs(&test_input);
        assert_eq!(no_small_paths, 226);
    }

    #[test]
    fn test_part2() {
        let test_input = parse(TEST_INPUT);
        let (no_small_paths, small_paths) = bfs(&test_input);
        assert_eq!(no_small_paths + small_paths, 3509);
    }
}
