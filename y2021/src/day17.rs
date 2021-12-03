const INPUT: &str = include_str!("./assets/day3.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    format!("  Part 1: {}\n  Part 2: {}", part1(&input), part2(&input))
}

pub fn part1(input: &[usize]) -> usize {
    todo!()
}
pub fn part2(input: &[usize]) -> usize {
    todo!()
}

fn parse(input: &str) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "";

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
