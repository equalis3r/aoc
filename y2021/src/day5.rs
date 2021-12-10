use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("./assets/day5.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    let mut grid: Grid = HashMap::new();
    format!(
        "  Part 1: {}\n  Part 2: {}",
        part1(&input, &mut grid),
        part2(&input, &mut grid)
    )
}

pub fn part1(lines: &[Line], grid: &mut Grid) -> usize {
    lines
        .iter()
        .filter(|line| !line.is_vertical())
        .for_each(|line| {
            line.points_in_between()
                .for_each(|(i, j)| *grid.entry([i, j]).or_insert(0) += 1)
        });
    grid.values().filter(|&val| *val >= 2).count()
}

pub fn part2(lines: &[Line], grid: &mut Grid) -> usize {
    lines
        .iter()
        .filter(|line| line.is_vertical())
        .for_each(|line| {
            line.points_in_between()
                .into_iter()
                .for_each(|(i, j)| *grid.entry([i, j]).or_insert(0) += 1)
        });
    grid.values().filter(|&val| *val >= 2).count()
}

pub struct Line {
    start: [u32; 2],
    end: [u32; 2],
}

impl Line {
    fn is_vertical(&self) -> bool {
        (self.start[0] != self.end[0]) && (self.start[1] != self.end[1])
    }

    fn points_in_between(&self) -> Box<dyn Iterator<Item = (u32, u32)> + 'static> {
        let xs: Vec<u32> = if self.start[0] < self.end[0] {
            (self.start[0]..=self.end[0]).collect()
        } else {
            (self.end[0]..=self.start[0]).rev().collect()
        };
        let ys: Vec<u32> = if self.start[1] < self.end[1] {
            (self.start[1]..=self.end[1]).collect()
        } else {
            (self.end[1]..=self.start[1]).rev().collect()
        };
        if xs.len() == ys.len() {
            Box::new(xs.into_iter().zip(ys.into_iter()))
        } else {
            Box::new(
                ys.into_iter()
                    .flat_map(move |y| xs.clone().into_iter().map(move |x| (x, y))),
            )
        }
    }
}

type Grid = HashMap<[u32; 2], u32>;

fn parse(input: &str) -> Vec<Line> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    }
    RE.captures_iter(input)
        .map(|cap| {
            cap.iter()
                .skip(1)
                .flatten()
                .filter_map(|cap| cap.as_str().parse().ok())
                .collect::<Vec<u32>>()
        })
        .map(|c| Line {
            start: [c[0], c[1]],
            end: [c[2], c[3]],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        let test_input = parse(TEST_INPUT);
        let mut grid: Grid = HashMap::new();
        assert_eq!(part1(&test_input, &mut grid), 5);
    }

    #[test]
    fn test_part2() {
        let test_input = parse(TEST_INPUT);
        let mut grid: Grid = HashMap::new();
        part1(&test_input, &mut grid);
        assert_eq!(part2(&test_input, &mut grid), 12);
    }
}
