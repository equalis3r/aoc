const INPUT: &str = include_str!("./assets/day5.txt");

pub fn solve() -> String {
    format!(
        "  Part 1: {}\n  Part 2: {}",
        INPUT.lines().map(|l| part_1(l)).filter(|&x| x).count(),
        INPUT.lines().map(|l| part_2(l)).filter(|&x| x).count(),
    )
}

pub fn part_1(haystack: &str) -> bool {
    contain_vowels(haystack) >= 3
        && contain_n_letters_in_row(haystack, 2)
        && !contain_certain_strings(haystack, &["ab", "cd", "pq", "xy"])
}

pub fn part_2(haystack: &str) -> bool {
    contain_twice(haystack) && contain_repeated(haystack)
}

fn contain_vowels(haystack: &str) -> usize {
    haystack
        .chars()
        .map(|letter| if "aeiou".contains(letter) { 1 } else { 0 })
        .sum()
}

fn contain_n_letters_in_row(haystack: &str, num: usize) -> bool {
    let mut count = 1;
    let mut prev_letter: char = '\n';
    for letter in haystack.chars() {
        if letter == prev_letter {
            count += 1;
            if count == num {
                return true;
            }
        } else {
            prev_letter = letter;
            count = 1;
        }
    }
    count == num
}

fn contain_certain_strings(haystack: &str, needles: &[&str]) -> bool {
    needles
        .iter()
        .any(|&sub_string| haystack.contains(sub_string))
}

fn contain_twice(haystack: &str) -> bool {
    let mut pivot = 0;
    while pivot < haystack.len() - 2 {
        if haystack[pivot + 2..].contains(&haystack[pivot..pivot + 2]) {
            return true;
        }
        pivot += 1;
    }
    false
}

fn contain_repeated(haystack: &str) -> bool {
    haystack
        .chars()
        .into_iter()
        .collect::<Vec<char>>()
        .windows(3)
        .any(|sub_string| sub_string[0] == sub_string[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice_string_part_1() {
        assert!(part_1("ugknbfddgicrmopn"));
        assert!(!part_1("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_nice_string_part_2() {
        assert!(part_2("qjhvhtzxzqqjkmpb"));
        assert!(!part_2("uurcxstgmygtbstg"));
    }
}
