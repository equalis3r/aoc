pub fn part1(input: &str) -> usize {
    parse(input).windows(2).filter(|x| x[0] < x[1]).count()
}
pub fn part2(input: &str) -> usize {
    parse(input)
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|y| y[0] < y[1])
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
    const INPUT: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 5);
    }
}
