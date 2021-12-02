use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let (a, b) = parse(input)
        .into_iter()
        .tuple_combinations()
        .find(|(a, b)| a + b == 2020)
        .expect("No combinations add up to 2020!");
    a * b
}

pub fn part2(input: &str) -> usize {
    let (a, b, c) = parse(input)
        .into_iter()
        .tuple_combinations()
        .find(|(a, b, c)| a + b + c == 2020)
        .expect("No combinations add up to 2020!");
    a * b * c
}

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}
