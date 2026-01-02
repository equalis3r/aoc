type Height = u8;
const INPUT: &str = include_str!("./assets/day9.txt");
const NEIGHBORS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
const MAX_HEIGHT: Height = b'9';

type Grid = Vec<Vec<Height>>;

pub fn solve() -> String {
    let mut grid = parse(INPUT);
    format!("  Part 1: {}\n  Part 2: {}", part1(&grid), part2(&mut grid))
}

pub fn part1(grid: &Grid) -> usize {
    let mut sum = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if NEIGHBORS.iter().all(|&(xx, yy)| {
                grid.get((y as isize + yy) as usize)
                    .and_then(|l| l.get((x as isize + xx) as usize))
                    .map(|n| cell < n)
                    .unwrap_or(true)
            }) {
                sum += (cell - b'0') as usize + 1;
            }
        }
    }
    sum
}

pub fn part2(grid: &mut Grid) -> usize {
    let mut basins = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            (grid[y][x] < MAX_HEIGHT).then(|| basins.push(find_basin_size(grid, x, y)));
        }
    }

    basins.sort_unstable();
    basins.iter().rev().take(3).product::<usize>()
}

fn find_basin_size(grid: &mut Grid, x: usize, y: usize) -> usize {
    grid[y][x] = b'9';
    NEIGHBORS
        .iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |acc, (x, y)| {
            match grid.get(y).and_then(|l| l.get(x)).map(|&n| n < b'9') {
                Some(true) => acc + find_basin_size(grid, x, y),
                _ => acc,
            }
        })
}

fn parse(input: &str) -> Grid {
    input
        .trim()
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|l| l.to_vec())
        .collect::<Vec<_>>()
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
        let grid = parse(TEST_INPUT);
        assert_eq!(part1(&grid), 15);
    }

    #[test]
    fn test_part2() {
        let mut grid = parse(TEST_INPUT);
        assert_eq!(part2(&mut grid), 1134);
    }
}
