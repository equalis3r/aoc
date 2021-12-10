const INPUT: &str = include_str!("./assets/day2.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    format!("  Part 1: {}\n  Part 2: {}", part1(&input), part2(&input))
}

fn part1(input: &[Command]) -> u32 {
    let (x, y) = input.iter().fold((0, 0), |(x, y), com| match com {
        Command::Forward(amount) => (x + amount, y),
        Command::Down(amount) => (x, y + amount),
        Command::Up(amount) => (x, y - amount),
    });
    x * y
}

fn part2(input: &[Command]) -> u32 {
    let (_, x, y) = input.iter().fold((0, 0, 0), |(aim, x, y), com| match com {
        Command::Forward(amount) => (aim, x + amount, y + aim * amount),
        Command::Down(amount) => (aim + amount, x, y),
        Command::Up(amount) => (aim - amount, x, y),
    });
    x * y
}

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| {
            let mut tokens = l.split(' ');
            match tokens.next() {
                Some(command) => {
                    let amount = match tokens.next() {
                        Some(num) => num.parse().unwrap(),
                        None => panic!("For line {}, size is expeted!", l),
                    };
                    match command {
                        "forward" => Command::Forward(amount),
                        "down" => Command::Down(amount),
                        "up" => Command::Up(amount),
                        _ => panic!("Command kind {} is unknown!", command),
                    }
                }
                None => panic!("For line {}, command is expected!", l),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn test_part1() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(part1(&test_input), 150);
    }

    #[test]
    fn test_part2() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(part2(&test_input), 900);
    }
}
