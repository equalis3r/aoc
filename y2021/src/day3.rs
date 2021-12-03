const INPUT: &str = include_str!("./assets/day3.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    format!(
        "  Part 1: {}\n  Part 2: {}",
        part1(&input, 12),
        part2(&input, 12),
    )
}

pub fn part1(input: &[usize], b: usize) -> usize {
    let gamma = (0..b)
        .rev()
        .into_iter()
        .map(|pos| most_common_bit_at_pos(input, pos).to_string())
        .collect::<String>();
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    gamma * flip_b_bits(gamma, b)
}

pub fn part2(input: &[usize], b: usize) -> usize {
    oxygen_rating(input, b) * co2_rating(input, b)
}

fn oxygen_rating(input: &[usize], mut b: usize) -> usize {
    let mut temp = input.clone().to_vec();
    while temp.len() > 1 {
        b -= 1;
        temp = temp
            .iter()
            .copied()
            .filter(|&x| find_bit_at_pos(x, b) == most_common_bit_at_pos(&temp, b))
            .collect::<Vec<_>>();
    }
    temp[0]
}

fn co2_rating(input: &[usize], mut b: usize) -> usize {
    let mut temp = input.clone().to_vec();
    while temp.len() > 1 {
        b -= 1;
        temp = temp
            .iter()
            .copied()
            .filter(|&x| find_bit_at_pos(x, b) == (1 - most_common_bit_at_pos(&temp, b)))
            .collect::<Vec<_>>();
    }
    temp[0]
}

fn most_common_bit_at_pos(input: &[usize], pos: usize) -> usize {
    let bit_1s = input
        .into_iter()
        .fold(0, |acc, &x| acc + find_bit_at_pos(x, pos));
    (bit_1s >= input.len() - bit_1s) as usize
}

fn find_bit_at_pos(n: usize, pos: usize) -> usize {
    n >> pos & 1
}

fn flip_b_bits(n: usize, b: usize) -> usize {
    (n) ^ ((1usize << (b)) - 1)
}

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|x| usize::from_str_radix(x, 2))
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn test_part1() {
        let parse_test_input = parse(TEST_INPUT);
        assert_eq!(part1(&parse_test_input, 5), 198);
    }

    #[test]
    fn test_part2() {
        let parse_test_input = parse(TEST_INPUT);
        assert_eq!(part2(&parse_test_input, 5), 230);
    }
}
