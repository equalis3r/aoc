use std::collections::HashMap;

const INPUT: &str = include_str!("./assets/day6.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    format!("  Part 1: {}\n  Part 2: {}", part_1(&input), part_2(&input))
}

fn part_1(input: &HashMap<&str, Vec<&str>>) -> usize {
    part_1_calc(input, "COM", 0)
}

fn part_1_calc(objects: &HashMap<&str, Vec<&str>>, planet: &str, lvl: usize) -> usize {
    match objects.get(planet) {
        Some(orbits) => {
            lvl + orbits
                .iter()
                .map(|pl| part_1_calc(objects, pl, lvl + 1))
                .sum::<usize>()
        }
        None => lvl,
    }
}

fn part_2(input: &HashMap<&str, Vec<&str>>) -> usize {
    part_2_calc(input, "YOU", "SAN", 0)
}

fn part_2_calc(objects: &HashMap<&str, Vec<&str>>, planet: &str, dest: &str, lvl: usize) -> usize {
    if planet == dest {
        lvl
    } else {
        match objects.get(planet) {
            Some(orbits) => orbits
                .iter()
                .map(|pl| part_2_calc(objects, pl, dest, lvl + 1))
                .sum::<usize>(),
            None => 0,
        }
    }
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let all_pairs = input
        .lines()
        .map(|val| {
            let pairs = val.split(')').collect::<Vec<&str>>();
            [pairs[0], pairs[1]]
        })
        .collect::<Vec<[&str; 2]>>();
    let mut objects: HashMap<&str, Vec<&str>> = HashMap::new();
    for pairs in all_pairs {
        objects
            .entry(pairs[0])
            .and_modify(|orbits| orbits.push(pairs[1]))
            .or_insert(vec![pairs[1]]);
    }
    objects
}

fn parse_part_2(input: &str) -> HashMap<&str, Vec<&str>> {
    let all_pairs = input
        .lines()
        .map(|val| {
            let pairs = val.split(')').collect::<Vec<&str>>();
            [pairs[0], pairs[1]]
        })
        .collect::<Vec<[&str; 2]>>();
    let mut objects: HashMap<&str, Vec<&str>> = HashMap::new();
    for pairs in all_pairs {
        objects
            .entry(pairs[0])
            .and_modify(|orbits| orbits.push(pairs[1]))
            .or_insert(vec![pairs[1]]);

        objects
            .entry(pairs[1])
            .and_modify(|orbits| orbits.push(pairs[0]))
            .or_insert(vec![pairs[0]]);
    }
    objects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        assert_eq!(part_1(&parse(test_input)), 42);
    }

    #[test]
    fn test_part_2() {
        let test_input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        assert_eq!(part_2(&parse_part_2(test_input)), 4);
    }
}
