const INPUT: &str = include_str!("./assets/day1.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    format!("  Part 1: {}\n  Part 2: {}", part1(&input), part2(&input))
}

pub fn part1(input: &[usize]) -> usize {
    input.windows(2).filter(|x| x[0] < x[1]).count()
}
pub fn part2(input: &[usize]) -> usize {
    input
        .windows(3)
        .map(|x| [x[0], x[2]])
        .collect::<Vec<[usize; 2]>>()
        .windows(2)
        .filter(|y| y[0][0] < y[1][1])
        .count()
}

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn test_part1() {
        let parse_test_input = parse(TEST_INPUT);
        assert_eq!(part1(&parse_test_input), 7);
    }

    #[test]
    fn test_part2() {
        let parse_test_input = parse(TEST_INPUT);
        assert_eq!(part2(&parse_test_input), 5);
    }
}
