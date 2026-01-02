const INPUT: &str = include_str!("./assets/day2.txt");

pub fn solve() -> String {
    let input = parse(INPUT);
    format!("  Part 1: {}\n  Part 2: {}", part_1(&input), part_2(&input))
}

pub struct Dimension {
    length: usize,
    width: usize,
    height: usize,
}

impl Dimension {
    pub fn surface_area(&self) -> usize {
        2 * (self.length * self.width + self.length * self.height + self.width * self.height)
    }

    pub fn smallest_area(&self) -> usize {
        [
            self.length * self.width,
            self.length * self.height,
            self.width * self.height,
        ]
        .iter()
        .fold(usize::MAX, |a, &b| a.min(b))
    }

    pub fn smallest_perimeter(&self) -> usize {
        [
            2 * (self.length + self.width),
            2 * (self.length + self.height),
            2 * (self.width + self.height),
        ]
        .iter()
        .fold(usize::MAX, |a, &b| a.min(b))
    }

    pub fn cubic_volume(&self) -> usize {
        self.length * self.width * self.height
    }
}

pub fn part_1(input: &[Dimension]) -> usize {
    input
        .iter()
        .map(|dim| dim.surface_area() + dim.smallest_area())
        .sum()
}

pub fn part_2(input: &[Dimension]) -> usize {
    input
        .iter()
        .map(|dim| dim.cubic_volume() + dim.smallest_perimeter())
        .sum()
}

fn parse(input: &str) -> Vec<Dimension> {
    input
        .lines()
        .map(|dim| {
            let mut dim = dim.split('x').map(|val| val.parse::<usize>().unwrap());
            Dimension {
                length: dim.next().unwrap(),
                width: dim.next().unwrap(),
                height: dim.next().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "2x3x4\n1x1x10";

    #[test]
    fn test_part_1() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(part_1(&test_input), 101);
    }

    #[test]
    fn test_part_2() {
        let test_input = parse(TEST_INPUT);
        assert_eq!(part_2(&test_input), 48);
    }
}
