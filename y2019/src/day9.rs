use crate::machine;
const INPUT: &str = include_str!("./assets/day9.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    println!("Part 1:");
    let mut intcomp = machine::Machine {
        memory: [0; 2000],
        pointer_addr: 0,
        relative_base: 0,
    };
    intcomp.load_initial_memory(&input).unwrap();
    intcomp.run().unwrap();
    println!("Part 2:");
    String::new()
}

fn parse(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect::<Vec<_>>()
}
