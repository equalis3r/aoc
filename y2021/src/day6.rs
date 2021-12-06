const INPUT: &str = include_str!("./assets/day6.txt");

pub fn solve() -> String {
    let fishes = parse(INPUT);
    format!(
        "  Part 1: {}\n  Part 2: {}",
        part_d(&fishes, 80),
        part_d(&fishes, 256)
    )
}

pub fn part_d(fishes: &[usize], day: usize) -> u64 {
    // Create an array represents the number of fishes that
    // will give birth after x days.
    // After each day, rotate the array to the left by 1.
    // New small fishes are shifted to last DOW already.
    // The old number of big fishes needs to be added back to DOW 6.
    // At any day, the sum of the array gives the total number of fishes.
    let mut dow: [u64; 9] = [0; 9];
    fishes.iter().for_each(|&f| dow[f] += 1);
    for _ in 0..day {
        dow.rotate_left(1);
        dow[6] += dow[8];
    }
    dow.iter().sum()
}

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        let parse_test_input = parse(TEST_INPUT);
        assert_eq!(part_d(&parse_test_input, 80), 5934);
    }

    #[test]
    fn test_part2() {
        let parse_test_input = parse(TEST_INPUT);
        assert_eq!(part_d(&parse_test_input, 256), 26984457539);
    }
}
