const INPUT: &str = include_str!("./assets/day1.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    format!("  Part 1: {}\n  Part 2: {}", part_1(&input), part_2(&input))
}

fn part_1(input: &[i32]) -> i32 {
    input.iter().fold(0i32, |acc, &val| acc + { val / 3 - 2 })
}

fn part_2(input: &[i32]) -> i32 {
    input.iter().fold(0i32, |acc, &val| acc + part_2_calc(val))
}

fn part_2_calc(input: i32) -> i32 {
    let val = input / 3 - 2;
    if val <= 0 {
        0
    } else {
        val + part_2_calc(val)
    }
}
fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|val| val.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = "100756";
        assert_eq!(part_1(&parse(test_input)), 33583);
    }

    #[test]
    fn test_part_2() {
        let test_input = "100756";
        assert_eq!(part_2(&parse(test_input)), 50346);
    }
}
