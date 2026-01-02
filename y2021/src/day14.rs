use std::mem;

const INPUT: &str = include_str!("./assets/day14.txt");

pub fn solve() -> String {
    format!(
        "  Part 1: {}\n  Part 2: {}",
        difference_of_max_and_min(INPUT, 10),
        difference_of_max_and_min(INPUT, 40)
    )
}

fn difference_of_max_and_min(s: &str, iterations: usize) -> u64 {
    let (base, rules) = s.split_once("\n\n").unwrap();

    let base = base.as_bytes().to_vec();
    let mut rules = rules
        .lines()
        .map(|l| {
            let (k, t) = l.split_once(" -> ").unwrap();
            let (k, t) = (k.as_bytes(), t.as_bytes()[0]);
            ([k[0], k[1]], [k[0], t])
        })
        .collect::<Vec<_>>();
    rules.sort_unstable_by_key(|r| r.0);
    let rule = rules
        .iter()
        .map(|r| {
            (
                r.0,
                rules.binary_search_by_key(&r.1, |r| r.0).unwrap(),
                rules
                    .binary_search_by_key(&[r.1[1], r.0[1]], |r| r.0)
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let (mut num, mut next) = (vec![0; rule.len()], vec![0; rule.len()]);
    base.windows(2)
        .for_each(|key| num[rule.binary_search_by_key(&key, |r| &r.0).unwrap()] += 1);

    (0..iterations).for_each(|_| {
        num.iter_mut().zip(&rule).for_each(|(n, r)| {
            next[r.1] += *n;
            next[r.2] += *n;
            *n = 0;
        });
        mem::swap(&mut num, &mut next);
    });

    let mut occur = [0; (b'Z' - b'A') as usize];
    occur[(base.last().unwrap() - b'A') as usize] += 1;
    rule.iter()
        .zip(num)
        .for_each(|(r, n)| occur[(r.0[0] - b'A') as usize] += n);

    let (min, max) = occur
        .iter()
        .filter(|&&n| n != 0)
        .fold((u64::MAX, 0), |(min, max), &n| (min.min(n), max.max(n)));
    max - min
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part1() {
        assert_eq!(difference_of_max_and_min(TEST_INPUT, 10), 1588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(difference_of_max_and_min(TEST_INPUT, 40), 2188189693529);
    }
}
