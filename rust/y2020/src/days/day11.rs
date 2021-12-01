use std::fmt;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Floor
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Floor => '.',
            Self::EmptySeat => 'L',
            Self::OccupiedSeat => '#',
        };
        write!(f, "{}", c)
    }
}

struct Map<T> {
    size: Vec2,
    tiles: Vec<T>,
}

impl<T> Map<T>
where
    T: Default + Copy,
{
    fn new(size: Vec2) -> Self {
        let num_tiles = size.x * size.y;
        Self {
            size,
            tiles: (0..num_tiles)
                .into_iter()
                .map(|_| Default::default())
                .collect(),
        }
    }

    fn index(&self, pos: Vec2) -> Option<usize> {
        if (0..self.size.x).contains(&pos.x) && (0..self.size.y).contains(&pos.y) {
            Some(pos.x + pos.y * self.size.x)
        } else {
            None
        }
    }

    fn get(&self, pos: Vec2) -> Option<T> {
        self.index(pos).map(|i| self.tiles[i])
    }

    fn set(&mut self, pos: Vec2, tile: T) {
        if let Some(i) = self.index(pos) {
            self.tiles[i] = tile;
        }
    }

    fn neighbor_position(&self, pos: Vec2) -> impl Iterator<Item = Vec2> {
        (-1..=1)
            .map(|dx| (-1..=1).map(|dy| (dx, dy)))
            .flatten()
            .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
            .map(|(dx, dy)| Vec2 {
                x: pos.x + dx,
                y: pos.y + dy,
            })
    }
}
