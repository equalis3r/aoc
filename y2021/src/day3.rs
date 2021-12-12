use std::collections::BTreeMap;
const INPUT: &str = include_str!("./assets/day3.txt");

pub fn solve() -> String {
    format!("  Part 1: {}\n  Part 2: {}", part1(INPUT), part2(INPUT),)
}

fn part1(s: &str) -> u64 {
    let mut map = BTreeMap::<_, i32>::new();

    for l in clean_lines(s) {
        for (p, c) in l.chars().enumerate() {
            let delta = if c == '1' { 1 } else { -1 };
            *map.entry(p).or_default() += delta;
        }
    }

    let mut gamma = 0u64;
    let mut mask = 0;

    for (_p, c) in map {
        assert_ne!(c, 0, "No majority");

        gamma <<= 1;

        if c > 0 {
            gamma |= 1;
        }

        mask <<= 1;
        mask |= 1;
    }

    let omega = !gamma & mask;
    omega * gamma
}

fn part2(s: &str) -> u64 {
    fn delve<'a>(lines: &[&'a str], prefer_one: bool, depth: usize) -> &'a str {
        // Exit if we only have one string
        if let Some((one, rest)) = lines.split_first() {
            if rest.is_empty() {
                return one;
            }
        }

        let (bit_0, bit_1): (Vec<_>, Vec<_>) = lines
            .iter()
            .partition(|l| l.chars().nth(depth) == Some('0'));

        use std::cmp::Ordering::*;
        let selected = match (bit_0.len().cmp(&bit_1.len()), prefer_one) {
            (Less, true) => bit_1,
            (Equal, true) => bit_1,
            (Greater, true) => bit_0,

            (Less, false) => bit_0,
            (Equal, false) => bit_0,
            (Greater, false) => bit_1,
        };

        delve(&selected, prefer_one, depth + 1)
    }

    let lines: Vec<_> = clean_lines(s).collect();
    let oxygen = delve(&lines, true, 0);
    let co2 = delve(&lines, false, 0);

    let oxygen = u64::from_str_radix(oxygen, 2).expect("Not binary");
    let co2 = u64::from_str_radix(co2, 2).expect("Not binary");

    oxygen * co2
}

fn clean_lines(s: &str) -> impl Iterator<Item = &str> {
    s.lines().map(str::trim).filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 230);
    }
}
