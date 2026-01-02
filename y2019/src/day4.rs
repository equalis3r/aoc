const INPUT: &str = include_str!("./assets/day4.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    format!("  Part 1: {}\n  Part 2: {}", part_1(&input), part_2(&input))
}

fn part_1(input: &[u32; 2]) -> usize {
    (input[0]..=input[1])
        .filter(|&val| part_1_calc(val))
        .count()
}

fn part_1_calc(input: u32) -> bool {
    let mut double_cond = false;
    let mut increasing_cond = true;
    let mut prev_digit = '0';
    for cur_digit in input.to_string().chars() {
        double_cond = double_cond || (prev_digit == cur_digit);
        increasing_cond = increasing_cond
            && (prev_digit.to_digit(10).unwrap() <= cur_digit.to_digit(10).unwrap());
        prev_digit = cur_digit;
    }
    double_cond && increasing_cond
}

fn part_2(input: &[u32; 2]) -> usize {
    (input[0]..=input[1])
        .filter(|&val| part_2_calc(val))
        .count()
}

fn part_2_calc(input: u32) -> bool {
    let mut double_cond = false;
    let mut increasing_cond = true;
    let mut even_cond = false;
    let mut count = 1;
    let mut prev_digit = '0';
    for cur_digit in input.to_string().chars() {
        if prev_digit == cur_digit {
            count += 1;
            double_cond = true;
        } else {
            if count == 2 {
                even_cond = true;
            }
            count = 1;
        }
        if prev_digit.to_digit(10).unwrap() > cur_digit.to_digit(10).unwrap() {
            increasing_cond = false;
        }
        prev_digit = cur_digit;
    }
    if count == 2 {
        even_cond = true;
    }
    double_cond && increasing_cond && even_cond
}
fn parse(input: &str) -> [u32; 2] {
    let mut res = [0; 2];
    let mut numbers = input.lines();
    res[0] = numbers.next().unwrap().parse().unwrap();
    res[1] = numbers.next().unwrap().parse().unwrap();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = [122345; 2];
        assert_eq!(part_1(&test_input), 1);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&[112233; 2]), 1);
        assert_eq!(part_2(&[123444; 2]), 0);
        assert_eq!(part_2(&[112224; 2]), 1);
    }
}
