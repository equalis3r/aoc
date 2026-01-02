use md5;

const INPUT: &str = include_str!("./assets/day4.txt");

pub fn solve() -> String {
    format!(
        "  Part 1: {}\n  Part 2: {}",
        hashing(INPUT.trim(), 5),
        hashing(INPUT.trim(), 6)
    )
}

pub fn hashing(secret_key: &str, num_zeroes: usize) -> usize {
    let mut input = 1;
    while !check_zeroes(gen_key(secret_key, input), num_zeroes) {
        input += 1;
    }
    input
}

fn check_zeroes(hash: String, num: usize) -> bool {
    if hash.len() < 5 {
        false
    } else {
        let first_num_characters = &hash[..num];
        match first_num_characters.parse::<usize>() {
            Ok(num) => num == 0,
            Err(_) => false,
        }
    }
}

fn gen_key(secret_key: &str, input: usize) -> String {
    format!("{:x}", md5::compute(format!("{}{}", secret_key, input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five_zeroes() {
        assert_eq!(hashing("abcdef".trim(), 5), 609043);
    }
}
