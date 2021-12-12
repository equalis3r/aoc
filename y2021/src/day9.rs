use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("./assets/day9.txt");
const MAX_HEIGHT: Height = 9;
type Coord = (usize, usize);
type Height = u8;

#[derive(Debug)]
pub struct Grid {
    max_x: usize,
    max_y: usize,
    points: HashMap<Coord, Height>,
}

pub fn solve() -> String {
    let input = parse(INPUT);
    format!("  Part 1: {}\n  Part 2: {}", part1(&input), part2(&input))
}

pub fn part1(grid: &Grid) -> u32 {
    find_minimums(grid).fold(0, |acc, (_, v)| acc + v as u32 + 1)
}

pub fn part2(grid: &Grid) -> usize {
    // find_minimums return iterator so it will be consumed and thus
    // there is no need for a global visited
    let mut sizes: Vec<_> = find_minimums(grid)
        .map(|(c, _)| c)
        .map(|c| find_basin_size(grid, c))
        .collect();
    sizes.sort_unstable();
    sizes.iter().rev().take(3).product()
}

fn find_minimums(grid: &Grid) -> impl Iterator<Item = (Coord, Height)> + '_ {
    itertools::iproduct!(0..=grid.max_x, 0..=grid.max_y).filter_map(move |c| {
        let v = grid.points[&c];
        get_neighbors(grid, c).all(|(_, t)| t > v).then(|| (c, v))
    })
}

fn find_basin_size(grid: &Grid, start: Coord) -> usize {
    let mut to_visit = vec![start];
    let mut visited = HashSet::new();

    while let Some(c) = to_visit.pop() {
        visited.insert(c);
        for (nc, nv) in get_neighbors(grid, c) {
            if nv != MAX_HEIGHT && !visited.contains(&nc) {
                to_visit.push(nc)
            }
        }
    }
    visited.len()
}

fn get_neighbors(grid: &Grid, (x, y): Coord) -> impl Iterator<Item = (Coord, Height)> {
    let same_y = |x| grid.points.get(&(x, y)).map(|&v| ((x, y), v));
    let same_x = |y| grid.points.get(&(x, y)).map(|&v| ((x, y), v));

    let l = x.checked_sub(1).and_then(same_y);
    let r = x.checked_add(1).and_then(same_y);
    let u = y.checked_sub(1).and_then(same_x);
    let d = y.checked_add(1).and_then(same_x);

    itertools::chain!(l, r, u, d)
}

fn parse(input: &str) -> Grid {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut points: HashMap<Coord, Height> = HashMap::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let c = c.to_digit(10).unwrap() as Height;
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
    const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part1() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(part1(&test_input), 15);
    }

    #[test]
    fn test_part2() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(part2(&test_input), 1134);
    }
}
