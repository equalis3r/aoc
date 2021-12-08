const INPUT: &str = include_str!("./assets/day8.txt");

pub fn solve() -> String {
    let mut input = parse(INPUT);
    format!(
        "  Part 1: {}\n  Part 2: {}",
        part1(&input),
        part2(&mut input)
    )
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
pub fn part2(input: &mut [Entry]) -> u32 {
    let answer = input
        .iter_mut()
        .map(|entry| {
            entry.char_mappings();
            entry
                .output_values
                .iter()
                .enumerate()
                .map(|(i, val)| 10u32.pow(3 - i as u32) * entry.digit_decode(val))
                .sum::<u32>()
        })
        .sum();
    answer
}

#[derive(Debug)]
pub struct Entry<'a> {
    wirepos: [char; 7],
    input_signals: [&'a str; 10],
    output_values: [&'a str; 4],
}

impl Entry<'_> {
    pub fn char_mappings(&mut self) {
        let mut digits = [""; 10];
        self.input_signals
            .iter()
            .for_each(|signal| match signal.len() {
                2 => digits[1] = signal,
                3 => digits[7] = signal,
                4 => digits[4] = signal,
                5 => {
                    if digits[2] == "" {
                        digits[2] = signal
                    } else if digits[3] == "" {
                        digits[3] = signal
                    } else {
                        digits[5] = signal
                    }
                }
                6 => {
                    if digits[0] == "" {
                        digits[0] = signal
                    } else if digits[6] == "" {
                        digits[6] = signal
                    } else {
                        digits[9] = signal
                    }
                }
                7 => digits[8] = signal,
                _ => panic!("Length of the signal is not correct!"),
            });
        self.wirepos[0] = digits[7]
            .chars()
            .filter(|&c| !digits[1].contains(c))
            .next()
            .unwrap();
        digits[4]
            .chars()
            .filter(|&c| !digits[1].contains(c))
            .collect::<Vec<char>>()
            .clone()
            .windows(2)
            .for_each(|c| {
                if [digits[2], digits[3], digits[5]]
                    .iter()
                    .all(|d| d.contains(c[0]))
                {
                    self.wirepos[1] = c[1];
                    self.wirepos[3] = c[0];
                } else {
                    self.wirepos[1] = c[0];
                    self.wirepos[3] = c[1];
                }
            });
        let two_five = digits[1].chars().collect::<Vec<char>>();
        [digits[2], digits[3], digits[5]].iter().for_each(|d| {
            if d.contains(self.wirepos[1]) {
                if d.contains(two_five[1]) {
                    self.wirepos[2] = two_five[0];
                    self.wirepos[5] = two_five[1];
                } else {
                    self.wirepos[2] = two_five[1];
                    self.wirepos[5] = two_five[0];
                }
            }
        });
        let four_six = "abcdefg"
            .chars()
            .filter(|c| !self.wirepos.contains(c))
            .collect::<Vec<char>>();
        [digits[2], digits[3], digits[5]].iter().for_each(|d| {
            if d.contains(self.wirepos[1]) {
                if d.contains(four_six[0]) {
                    self.wirepos[4] = four_six[1];
                    self.wirepos[6] = four_six[0];
                } else {
                    self.wirepos[4] = four_six[0];
                    self.wirepos[6] = four_six[1];
                }
            }
        });
    }

    pub fn digit_decode(&self, wires: &str) -> u32 {
        match wires.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            5 => {
                if wires.contains(self.wirepos[1]) {
                    5
                } else if wires.contains(self.wirepos[4]) {
                    2
                } else {
                    3
                }
            }
            6 => {
                if !wires.contains(self.wirepos[3]) {
                    0
                } else if !wires.contains(self.wirepos[2]) {
                    6
                } else {
                    9
                }
            }
            7 => 8,
            _ => panic!("Length of the signal is not correct!"),
        }
    }
}

fn parse(input: &str) -> Vec<Entry<'_>> {
    input
        .lines()
        .map(|entry| {
            let mut entry = entry.split('|');
            Entry {
                wirepos: [' '; 7],
                input_signals: entry
                    .next()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .collect::<Vec<&str>>()
                    .try_into()
                    .unwrap(),
                output_values: entry
                    .next()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .collect::<Vec<&str>>()
                    .try_into()
                    .unwrap(),
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
        let mut test_input = parse(TEST_INPUT);
        assert_eq!(part2(&mut test_input), 61229);
    }

    #[test]
    fn test_one_entry() {
        let mut test_input = parse(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        assert_eq!(part2(&mut test_input), 5353);
    }
}
