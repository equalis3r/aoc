use std::collections::{HashMap, HashSet};
use std::fmt;

const INPUT: &str = include_str!("./assets/day11.txt");
const THRESHOLD: u32 = 9;
type Coord = (usize, usize);
type Energy = u32;

pub fn solve() -> String {
    format!(
        "  Part 1: {}\n  Part 2: {}",
        part1(parse(INPUT), 100),
        part2(parse(INPUT))
    )
}

pub fn part1(mut grid: Grid, steps: usize) -> usize {
    (0..steps).map(|_| step_run(&mut grid)).sum()
}

pub fn part2(mut grid: Grid) -> usize {
    (1..)
        .find(|_| step_run(&mut grid) == grid.points.len())
        .unwrap()
}

pub fn step_run(grid: &mut Grid) -> usize {
    let mut to_visit = Vec::new();
    let mut flashed = HashSet::new();

    for (&c, v) in grid.points.iter_mut() {
        *v += 1;
        if *v > THRESHOLD {
            to_visit.push(c);
        }
    }

    while let Some(c) = to_visit.pop() {
        flashed.insert(c);

        for nc in get_neighbors(c) {
            if let Some(v) = grid.points.get_mut(&nc) {
                *v += 1;
                if *v > THRESHOLD && !flashed.contains(&nc) && !to_visit.contains(&nc) {
                    to_visit.push(nc);
                }
            }
        }
    }

    flashed
        .iter()
        .for_each(|c| *grid.points.get_mut(&c).unwrap() = 0);

    flashed.len()
}

fn get_neighbors((x, y): Coord) -> impl Iterator<Item = Coord> {
    let x0 = x.checked_sub(1);
    let x1 = Some(x);
    let x2 = x.checked_add(1);
    let y0 = y.checked_sub(1);
    let y1 = Some(y);
    let y2 = y.checked_add(1);

    itertools::chain!(
        x0.and_then(|x| Some((x, y0?))),
        x0.and_then(|x| Some((x, y1?))),
        x0.and_then(|x| Some((x, y2?))),
        x1.and_then(|x| Some((x, y0?))),
        x1.and_then(|x| Some((x, y2?))),
        x2.and_then(|x| Some((x, y0?))),
        x2.and_then(|x| Some((x, y1?))),
        x2.and_then(|x| Some((x, y2?))),
    )
}

pub struct Grid {
    max_x: usize,
    max_y: usize,
    points: HashMap<Coord, Energy>,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                write!(f, "{}", self.points[&(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Grid {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut points: HashMap<Coord, Energy> = HashMap::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let c = c.to_digit(10).unwrap() as Energy;
            points.insert((x, y), c);
            max_x = std::cmp::max(max_x, x);
        }
        max_y = std::cmp::max(max_y, y);
    }

    Grid {
        max_x,
        max_y,
        points,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part1() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(part1(test_input, 100), 1656);
    }

    #[test]
    fn test_part2() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(part2(test_input), 195);
    }
}
