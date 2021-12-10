const INPUT: &[u8] = include_bytes!("./assets/day10.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    format!("  Part 1: {}\n  Part 2: {}", part1(&input), part2(&input))
}

pub fn part1(input: &[&[u8]]) -> u32 {
    let mut scores: Vec<u32> = Vec::new();
    for line in input.iter() {
        let mut stack: Vec<u8> = Vec::new();
        for c in line.iter() {
            match c {
                b'(' | b'[' | b'{' | b'<' => stack.push(*c),
                _ => {
                    if !is_valid(stack.pop().unwrap(), *c) {
                        scores.push(score_closing(*c));
                        break;
                    }
                }
            }
        }
    }
    scores.iter().sum()
}
pub fn part2(input: &[&[u8]]) -> u64 {
    let mut scores: Vec<u64> = Vec::new();
    for line in input.iter() {
        let mut stack: Vec<u8> = Vec::new();
        let mut complete = true;
        for c in line.iter() {
            match c {
                b'(' | b'[' | b'{' | b'<' => stack.push(*c),
                _ => {
                    if !is_valid(stack.pop().unwrap(), *c) {
                        complete = false;
                        break;
                    }
                }
            }
        }
        if complete {
            scores.push(score_opening(&stack));
        }
    }
    scores.sort();
    scores[scores.len() / 2]
}

fn is_valid(opening: u8, closing: u8) -> bool {
    match closing {
        b')' => opening + 1 == closing,
        b']' | b'}' | b'>' => opening + 2 == closing,
        _ => panic!("Unrecognized closing character!"),
    }
}

fn score_opening(opening: &[u8]) -> u64 {
    opening.into_iter().rev().fold(0, |acc, o| {
        5 * acc
            + match o {
                b'(' => 1,
                b'[' => 2,
                b'{' => 3,
                b'<' => 4,
                _ => panic!("Unrecognized opening character!"),
            }
    })
}

fn score_closing(closing: u8) -> u32 {
    match closing {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => panic!("Unrecognized closing character!"),
    }
}

fn parse(input: &[u8]) -> Vec<&[u8]> {
    input.split(|&c| c == b'\n').collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &[u8] = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
        .as_bytes();

    #[test]
    fn test_part1() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(part1(&test_input), 26397);
    }

    #[test]
    fn test_part2() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(part2(&test_input), 288957);
    }
}
