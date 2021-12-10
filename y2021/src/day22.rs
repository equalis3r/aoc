const INPUT: &str = include_str!("./assets/day9.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    format!("  Part 1: {}\n  Part 2: {}", part1(&input), 0)
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
        let test_input = parse(TEST_INPUT);
        assert_eq!(part1(&test_input), 7);
    }

    #[test]
    fn test_part2() {
        todo!()
    }
}
