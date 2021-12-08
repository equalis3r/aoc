const INPUT: &str = include_str!("./assets/day8.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    format!("  Part 1: {}\n  Part 2: {}", part1(&input), part2(&input))
}

pub fn part1(input: &[Entry]) -> u32 {
    input
        .iter()
        .map(|entry| {
            entry
                .output_values
                .iter()
                .map(|output| match output.len() {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum()
}
pub fn part2(input: &[Entry]) -> u32 {
    input.iter().map(|entry| entry.decode_number()).sum()
}

#[derive(Debug)]
pub struct Entry<'a> {
    input_signals: Vec<&'a str>,
    output_values: Vec<&'a str>,
}

impl Entry<'_> {
    pub fn decode_number(&self) -> u32 {
        let pat1 = self
            .input_signals
            .iter()
            .find(|signal| signal.len() == 2)
            .unwrap();
        let pat2 = &self
            .input_signals
            .iter()
            .find(|signal| signal.len() == 4)
            .unwrap()
            .chars()
            .filter(|&c| !&pat1.contains(c))
            .collect::<String>()[..];
        self.output_values
            .iter()
            .enumerate()
            .map(|(i, signal)| 10u32.pow(3 - i as u32) * decode_digit(signal, pat1, pat2))
            .sum::<u32>()
    }
}

fn contains_str(a: &str, b: &str) -> bool {
    b.chars().all(|c| a.contains(c))
}

fn decode_digit(signal: &str, pat1: &str, pat2: &str) -> u32 {
    match signal.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => {
            if contains_str(signal, pat1) {
                3
            } else if contains_str(signal, pat2) {
                5
            } else {
                2
            }
        }
        6 => {
            if contains_str(signal, pat1) {
                if contains_str(signal, pat2) {
                    9
                } else {
                    0
                }
            } else {
                6
            }
        }
        7 => 8,
        _ => panic!("Length of the signal is not correct!"),
    }
}

fn parse(input: &str) -> Vec<Entry<'_>> {
    input
        .lines()
        .map(|entry| {
            let mut entry = entry.split('|');
            Entry {
                input_signals: entry
                    .next()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .collect::<Vec<&str>>(),
                output_values: entry
                    .next()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .collect::<Vec<&str>>(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(part1(&test_input), 26);
    }

    #[test]
    fn test_part2() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(part2(&test_input), 61229);
    }

    #[test]
    fn test_one_entry() {
        let test_input = parse(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        assert_eq!(part2(&test_input), 5353);
    }
}
