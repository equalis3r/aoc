pub fn part1(input: &str) -> usize {
    let record = parse(input)
        .into_iter()
        .fold(Record { aim: 0, x: 0, y: 0 }, |mut acc, com| {
            match com.kind {
                Kind::Forward => acc.x += com.size,
                Kind::Down => acc.y += com.size,
                Kind::Up => acc.y -= com.size,
            };
            acc
        });
    record.x * record.y
}

pub fn part2(input: &str) -> usize {
    let record = parse(input)
        .into_iter()
        .fold(Record { aim: 0, x: 0, y: 0 }, |mut acc, com| {
            match com.kind {
                Kind::Forward => {
                    acc.x += com.size;
                    acc.y += acc.aim * com.size;
                }
                Kind::Down => acc.aim += com.size,
                Kind::Up => acc.aim -= com.size,
            };
            acc
        });
    record.x * record.y
}

enum Kind {
    Forward,
    Down,
    Up,
}

struct Command {
    kind: Kind,
    size: usize,
}

struct Record {
    aim: usize,
    x: usize,
    y: usize,
}

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| {
            let mut tokens = l.split(' ');
            Command {
                kind: match tokens.next() {
                    Some(tok) => match tok {
                        "forward" => Kind::Forward,
                        "down" => Kind::Down,
                        "up" => Kind::Up,
                        _ => panic!("Command kind {} is unknown!", tok),
                    },
                    None => panic!("For line {}, command is expected!", l),
                },
                size: match tokens.next() {
                    Some(tok) => tok.parse().unwrap(),
                    None => panic!("For line {}, size is expeted!", l),
                },
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 900);
    }
}
