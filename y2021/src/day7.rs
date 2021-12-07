const INPUT: &str = include_str!("./assets/day7.txt");

pub fn solve() -> String {
    let mut input = parse(INPUT);
    format!(
        "  Part 1: {}\n  Part 2: {}",
        part1(&mut input),
        part2(&input)
    )
}

pub fn part1(input: &mut [i32]) -> i32 {
    let length = input.len();
    input.sort();
    let median = {
        if length % 2 == 1 {
            input[length / 2]
        } else {
            (input[length / 2 - 1] + input[length / 2]) / 2
        }
    };
    input.iter().map(|x| (x - median).abs()).sum()
}
pub fn part2(input: &[i32]) -> i32 {
    let avg = input.iter().sum::<i32>() / input.len() as i32;
    input
        .iter()
        .fold([0, 0], |acc, &x| {
            [acc[0] + cost_f(x, avg), acc[1] + cost_f(x, avg + 1)]
        })
        .into_iter()
        .min()
        .unwrap()
}

fn cost_f(x: i32, x0: i32) -> i32 {
    (x - x0).abs() * ((x - x0).abs() + 1) / 2
}

fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        let mut test_input = parse(TEST_INPUT);
        assert_eq!(part1(&mut test_input), 37);
    }

    #[test]
    fn test_part2() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(part2(&test_input), 168);
    }
}
