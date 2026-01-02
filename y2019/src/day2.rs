use crate::machine;
const INPUT: &str = include_str!("./assets/day2.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    format!(
        "   Part 1: {}\n    Part 2: {}",
        part1(&input, 12, 2),
        part2(&input, 19690720)
    )
}

fn parse(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect::<Vec<_>>()
}

fn part1(input: &[isize], noun: isize, verb: isize) -> isize {
    let mut intcomp = machine::Machine {
        memory: [0; 2000],
        pointer_addr: 0,
        relative_base: 0,
    };
    intcomp.memory[1] = noun;
    intcomp.memory[2] = verb;
    intcomp.load_initial_memory(input).unwrap();
    intcomp.run().unwrap();
    intcomp.memory[0]
}

fn part2(input: &[isize], output: isize) -> isize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if part1(input, noun, verb) == output {
                return 100 * noun + verb;
            }
        }
    }
    println!("Cannot find the combination of noun and verb to produce the required output!");
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let parsed_input = parse("1,9,10,3");
        assert_eq!(parsed_input, vec![1, 9, 10, 3]);
    }

    #[test]
    fn test_part1() {
        let input = parse("1,1,1,4,99,5,6,0,99");
        assert_eq!(part1(&input, 1, 1), 30);
    }
}
