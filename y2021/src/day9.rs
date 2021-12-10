const INPUT: &str = include_str!("./assets/day9.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    format!("  Part 1: {}\n  Part 2: {}", part1(&input), part2(&input))
}

pub fn part1(grid: &Grid) -> u32 {
    grid.points
        .iter()
        .enumerate()
        .map(|(i, &p)| {
            if get_adjacent(grid, i)
                .iter()
                .all(|&ind| grid.points[ind] > p)
            {
                p as u32 + 1
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(grid: &Grid) -> usize {
    let mut unvisited = grid
        .points
        .iter()
        .enumerate()
        .filter(|(_, &p)| p != 9)
        .map(|(i, _)| i as usize)
        .collect::<Vec<_>>();
    let mut count = 0;
    let mut basin_size: Vec<usize> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    while let Some(point) = unvisited.pop() {
        stack.push(point);
        while let Some(i) = stack.pop() {
            count += 1;
            unvisited.retain(|&x| x != i);
            get_adjacent(grid, i)
                .into_iter()
                .filter(|&ind| grid.points[ind] != 9)
                .for_each(|ind| {
                    if unvisited.contains(&ind) {
                        stack.push(ind);
                        unvisited.retain(|&x| x != ind);
                    }
                });
        }
        basin_size.push(count);
        count = 0;
    }
    basin_size.sort_by(|a, b| b.cmp(a));
    basin_size.iter().take(3).product()
}

#[derive(Debug)]
pub struct Grid {
    size: [usize; 2],
    points: Vec<u8>,
}

fn get_adjacent(grid: &Grid, i: usize) -> Vec<usize> {
    let x = i % grid.size[0];
    let y = i / grid.size[0];
    let left = if x > 0 { Some(i - 1) } else { None };
    let right = if x + 1 < grid.size[0] {
        Some(i + 1)
    } else {
        None
    };
    let up = if y > 0 { Some(i - grid.size[0]) } else { None };
    let down = if y + 1 < grid.size[1] {
        Some(i + grid.size[0])
    } else {
        None
    };
    [left, right, up, down]
        .into_iter()
        .filter_map(|p| p)
        .collect()
}

fn parse(input: &str) -> Grid {
    let arr = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect::<Vec<Vec<u8>>>();
    Grid {
        size: [arr[0].len(), arr.len()],
        points: arr.into_iter().flatten().collect(),
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
