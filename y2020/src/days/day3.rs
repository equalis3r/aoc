use ndarray::Array2;
use std::fmt;

fn main() -> anyhow::Result<()> {
    let m = Map::parse(include_str!("input.txt").trim().as_bytes())?;
    let num_trees = encounter_trees(&m, [3, 1]);
    println!("Part 1: We encountered {} trees.", num_trees);

    let slopes = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let answer = slopes
        .iter()
        .map(|&slope| encounter_trees(&m, slope))
        .product::<usize>();
    println!("Part 2: The answer is {}.", answer);
    Ok(())
}

fn encounter_trees(m: &Map, slope: Vec2) -> usize {
    (0..m.tiles.nrows())
        .into_iter()
        .map(|y| m.get([y * slope[1], y * slope[0]]))
        .fold(0usize, |acc, x| acc + x as usize)
}

type Vec2 = [usize; 2];

struct Map {
    tiles: Array2<u8>,
}

impl Map {
    fn new(size: Vec2) -> Self {
        Self {
            tiles: Array2::zeros(size),
        }
    }

    fn set(&mut self, pos: Vec2, tile: Option<u8>) -> anyhow::Result<()> {
        let pos = self.index(pos).unwrap();
        match tile {
            Some(b'.') => self.tiles[pos] = 0,
            Some(b'#') => self.tiles[pos] = 1,
            c => panic!("Expected '.' or '#', but got: {:?}", c),
        }
        Ok(())
    }

    fn index(&self, pos: Vec2) -> Option<Vec2> {
        if pos[0] >= self.tiles.nrows() {
            None
        } else {
            Some([pos[0], pos[1] % self.tiles.ncols()])
        }
    }
    fn get(&self, pos: Vec2) -> u8 {
        match self.index(pos) {
            Some(ind) => self.tiles[ind],
            None => 0,
        }
    }

    fn parse(input: &[u8]) -> anyhow::Result<Self> {
        let mut rows: usize = 1;
        let mut cols: usize = 0;
        for &c in input.iter() {
            if c == b'\n' {
                rows += 1;
                cols = 0;
            } else {
                cols += 1;
            }
        }

        let mut iter = input.iter().copied();
        let mut map = Map::new([rows, cols]);
        for row in 0..rows {
            for col in 0..cols {
                map.set([row, col], iter.next())?;
            }
            iter.next();
        }
        Ok(map)
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.tiles.rows() {
            for item in row {
                match item {
                    0 => write!(f, ".")?,
                    1 => write!(f, "#")?,
                    _ => write!(f, "E")?,
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
