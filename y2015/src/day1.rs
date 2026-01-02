const INPUT: &[u8] = include_bytes!("./assets/day1.txt");

pub fn solve() -> String {
    format!(
        "  Part 1: {}\n  Part 2: {}",
        part_1(&INPUT),
        part_2(&INPUT).unwrap()
    )
}

fn part_1(input: &[u8]) -> isize {
    input.iter().fold(0isize, |acc, &val| {
        acc + {
            if val == 40 {
                1
            } else if val == 41 {
                -1
            } else {
                0
            }
        }
    })
}

fn part_2(input: &[u8]) -> Option<usize> {
    let mut acc: isize = 0;
    for (id, &val) in input.iter().enumerate() {
        acc += {
            if val == 40 {
                1
            } else if val == 41 {
                -1
            } else {
                0
            }
        };
        if acc == -1 {
            return Some(id + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = ")())())".as_bytes();
        assert_eq!(part_1(&test_input), -3);
    }

    #[test]
    fn test_part_2() {
        let test_input = "()())".as_bytes();
        assert_eq!(part_2(&test_input), Some(5));
    }
}
