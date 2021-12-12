use std::collections::HashMap;

const INPUT: &str = include_str!("./assets/day5.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    let mut grid: Grid = HashMap::new();
    format!(
        "  Part 1: {}\n  Part 2: {}",
        solve_part(&input, &mut grid, false),
        solve_part(&input, &mut grid, true),
    )
}

pub fn solve_part(lines: &[Line], grid: &mut Grid, vertical: bool) -> usize {
    lines
        .iter()
        .filter(|line| line.is_vertical() == vertical)
        .for_each(|line| {
            line.points_in_between()
                .for_each(|(i, j)| *grid.entry([i, j]).or_insert(0) += 1)
        });
    grid.values().filter(|&val| *val >= 2).count()
}

type Grid = HashMap<[u32; 2], u32>;

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

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .flat_map(|l| {
            let (l, r) = l.split_once(" -> ")?;
            let (x1, y1) = l.split_once(',')?;
            let (x2, y2) = r.split_once(',')?;

            Some(Line {
                start: [x1.parse().unwrap(), y1.parse().unwrap()],
                end: [x2.parse().unwrap(), y2.parse().unwrap()],
            })
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
        assert_eq!(solve_part(&test_input, &mut grid, false), 5);
    }

    #[test]
    fn test_part2() {
        let test_input = parse(TEST_INPUT);
        let mut grid: Grid = HashMap::new();
        solve_part(&test_input, &mut grid, false);
        assert_eq!(solve_part(&test_input, &mut grid, true), 12);
    }
}
