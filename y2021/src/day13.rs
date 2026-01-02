use std::collections::HashSet;
const INPUT: &str = include_str!("./assets/day13.txt");

type Grid = HashSet<(usize, usize)>;

pub fn solve() -> String {
    let (grid, ins) = parse(INPUT);
    format!(
        "  Part 1: {}\n  Part 2: \n{}",
        run_instructions(&grid, &ins[..1]).len(),
        get_code(run_instructions(&grid, &ins))
    )
}

pub fn run_instructions(grid: &Grid, ins: &[Fold]) -> Grid {
    let mut cur_grid = grid.clone();
    for one_ins in ins {
        let mut next_grid = HashSet::new();
        match one_ins {
            Fold::X(ind) => {
                for &(x, y) in cur_grid.iter() {
                    if x > *ind {
                        next_grid.insert((ind * 2 - x, y));
                    } else if x < *ind {
                        next_grid.insert((x, y));
                    }
                }
            }
            Fold::Y(ind) => {
                for &(x, y) in cur_grid.iter() {
                    if y > *ind {
                        next_grid.insert((x, ind * 2 - y));
                    } else if y < *ind {
                        next_grid.insert((x, y));
                    }
                }
            }
        }
        cur_grid = next_grid;
    }
    cur_grid
}

fn get_code(grid: Grid) -> String {
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for (x, y) in grid.iter() {
        max_x = std::cmp::max(max_x, *x);
        max_y = std::cmp::max(max_y, *y);
    }
    let mut grid_string = vec![vec![' '; max_x + 1]; max_y + 1];
    for &(x, y) in grid.iter() {
        grid_string[y][x] = '#';
    }
    grid_string
        .into_iter()
        .map(|v| v.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

pub enum Fold {
    X(usize),
    Y(usize),
}

fn parse(input: &str) -> (Grid, Vec<Fold>) {
    let mut grid = Grid::new();
    let (dots, instructions) = input.split_once("\n\n").unwrap();

    for l in dots.lines() {
        let (x, y) = l.split_once(',').unwrap();
        grid.insert((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
    }

    let ins_list = instructions
        .lines()
        .map(|l| {
            let (ins_type, val) = l.split_once('=').unwrap();
            match ins_type {
                "fold along x" => Fold::X(val.parse::<usize>().unwrap()),
                "fold along y" => Fold::Y(val.parse::<usize>().unwrap()),
                _ => panic!("Invalid instruction!"),
            }
        })
        .collect();

    (grid, ins_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part1() {
        let (grid, ins) = parse(TEST_INPUT);
        assert_eq!(run_instructions(&grid, &ins[..1]).len(), 17);
    }
}
