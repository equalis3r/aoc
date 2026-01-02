use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("./assets/day3.txt");

pub fn solve() -> String {
    let mut houses = HashSet::<(isize, isize)>::new();
    format!(
        "  Part 1: {}\n  Part 2: {}",
        part_1(&INPUT, &mut houses),
        part_2(&INPUT)
    )
}

pub fn part_1(input: &[u8], houses: &mut HashSet<(isize, isize)>) -> usize {
    let mut cur: (isize, isize) = (0, 0);
    houses.insert(cur);
    for &val in input {
        if val == 60 {
            cur.0 -= 1;
        } else if val == 62 {
            cur.0 += 1;
        } else if val == 94 {
            cur.1 -= 1;
        } else if val == 118 {
            cur.1 += 1;
        } else if val == 10 {
            break;
        } else {
            panic!("Unrecognized direction!")
        }
        houses.insert(cur);
    }
    houses.len()
}

pub fn part_2(input: &[u8]) -> usize {
    let santa = input.iter().step_by(2).copied().collect::<Vec<_>>();
    let robot_santa = input.iter().skip(1).step_by(2).copied().collect::<Vec<_>>();
    let mut houses = HashSet::<(isize, isize)>::new();
    part_1(&santa, &mut houses);
    part_1(&robot_santa, &mut houses);
    houses.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let mut houses = HashSet::<(isize, isize)>::new();
        assert_eq!(part_1("^>v<".as_bytes(), &mut houses), 4);
        houses = HashSet::<(isize, isize)>::new();
        assert_eq!(part_1("^v^v^v^v^v".as_bytes(), &mut houses), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("^>v<".as_bytes()), 3);
        assert_eq!(part_2("^v^v^v^v^v".as_bytes()), 11);
    }
}
