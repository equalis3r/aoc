use ndarray::prelude::*;

const INPUT: &str = include_str!("./assets/day6.txt");

pub fn solve() -> String {
    let instruction = Instruction::parse(INPUT);
    format!(
        "  Part 1: {}\n  Part 2: {}",
        part_1(&instruction),
        part_2(&instruction),
    )
}

type Coord = [usize; 2];

pub enum Instruction {
    On([Coord; 2]),
    Off([Coord; 2]),
    Toggle([Coord; 2]),
}

pub fn part_1(input: &[Instruction]) -> usize {
    let mut grid = Array::from_elem((1000, 1000), 0);
    for ins in input {
        match ins {
            Instruction::On([pos1, pos2]) => grid
                .slice_mut(s![pos1[0]..=pos2[0], pos1[1]..=pos2[1]])
                .fill(1),
            Instruction::Off([pos1, pos2]) => grid
                .slice_mut(s![pos1[0]..=pos2[0], pos1[1]..=pos2[1]])
                .fill(0),
            Instruction::Toggle([pos1, pos2]) => grid
                .slice_mut(s![pos1[0]..=pos2[0], pos1[1]..=pos2[1]])
                .mapv_inplace(|val| 1 - val),
        }
    }
    grid.sum()
}

pub fn part_2(input: &[Instruction]) -> usize {
    let mut grid: Array<usize, _> = Array::from_elem((1000, 1000), 0);
    for ins in input {
        match ins {
            Instruction::On([pos1, pos2]) => grid
                .slice_mut(s![pos1[0]..=pos2[0], pos1[1]..=pos2[1]])
                .mapv_inplace(|val| val + 1),
            Instruction::Off([pos1, pos2]) => grid
                .slice_mut(s![pos1[0]..=pos2[0], pos1[1]..=pos2[1]])
                .mapv_inplace(|val| if val > 0 { val - 1 } else { val }),
            Instruction::Toggle([pos1, pos2]) => grid
                .slice_mut(s![pos1[0]..=pos2[0], pos1[1]..=pos2[1]])
                .mapv_inplace(|val| val + 2),
        }
    }
    grid.sum()
}

impl Instruction {
    fn parse_line(s: &str) -> anyhow::Result<Instruction> {
        peg::parser! {
          grammar parser() for str {
            rule number() -> usize
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule positions() -> [usize; 2]
                = first:number() "," second:number() { [first, second] }

            pub(crate) rule line() -> Instruction
                = "turn on " pos1:positions() " through " pos2:positions() {Instruction::On([pos1, pos2])} /
                "turn off " pos1:positions() " through " pos2:positions() {Instruction::Off([pos1, pos2])} /
                "toggle " pos1:positions() " through " pos2:positions() {Instruction::Toggle([pos1, pos2])}
          }
        }

        Ok(parser::line(s)?)
    }

    fn parse(input: &str) -> Vec<Instruction> {
        input
            .lines()
            .map(Instruction::parse_line)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }
}
