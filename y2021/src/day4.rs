const INPUT: &str = include_str!("./assets/day4.txt");

pub fn solve() -> String {
    let (numbers1, tables1) = parse(INPUT).unwrap();
    let (numbers2, tables2) = parse(INPUT).unwrap();
    format!(
        "  Part 1: {}\n  Part 2: {}",
        part1(numbers1, tables1),
        part2(numbers2, tables2)
    )
}

pub fn part1(numbers: Vec<i32>, mut tables: Vec<Table>) -> usize {
    let mut picked_number = numbers.iter();
    loop {
        let number = picked_number.next().unwrap();
        for table in tables.iter_mut() {
            table.check(*number);
        }
        if let Some(table) = tables.iter().filter(|x| x.check_bingo()).take(1).next() {
            break *number as usize * table.get_score() as usize;
        };
    }
}

pub fn part2(numbers: Vec<i32>, mut tables: Vec<Table>) -> usize {
    let mut picked_number = numbers.into_iter();
    loop {
        let number = picked_number.next().unwrap();
        for table in tables.iter_mut() {
            table.check(number);
        }
        let tables: Vec<Table> = tables
            .iter()
            .copied()
            .filter(|&x| !x.check_bingo())
            .collect();
        if tables.len() == 1 {
            let numbers: Vec<_> = picked_number.collect();
            break part1(numbers, tables);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Table {
    numbers: [i32; 25],
    check: i32,
    bingo: bool,
}

impl Table {
    fn from(input: Vec<i32>) -> Self {
        Self {
            numbers: input.try_into().unwrap(),
            check: 0,
            bingo: false,
        }
    }

    fn check(&mut self, val: i32) {
        if let Some(pos) = self.numbers.iter().position(|&x| x == val) {
            self.numbers[pos] = -1;
            self.check += 1;
        }
    }

    fn get_score(&self) -> i32 {
        self.numbers.iter().sum::<i32>() + self.check
    }

    fn check_bingo(&self) -> bool {
        if self.check < 5 {
            false
        } else {
            self.bingo || self.col_bingo() || self.row_bingo()
        }
    }

    fn row_bingo(&self) -> bool {
        (0..5)
            .into_iter()
            .filter(|x| self.numbers[(x * 5)..((x + 1) * 5)].iter().sum::<i32>() == -5)
            .count()
            >= 1
    }

    fn col_bingo(&self) -> bool {
        (0..5)
            .into_iter()
            .filter(|&n| {
                self.numbers
                    .iter()
                    .skip(n)
                    .step_by(5)
                    .copied()
                    .collect::<Vec<_>>()
                    .iter()
                    .sum::<i32>()
                    == -5
            })
            .count()
            >= 1
    }
}

fn parse(input: &str) -> anyhow::Result<(Vec<i32>, Vec<Table>)> {
    let mut string = input.trim().split("\n");
    let numbers: Vec<i32> = string
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let mut tables: Vec<Table> = Vec::new();
    while let Some(_) = string.next() {
        let table = string
            .by_ref()
            .take(5)
            .collect::<Vec<&str>>()
            .join(" ")
            .split_whitespace()
            .map(str::parse::<i32>)
            .collect::<Result<Vec<_>, _>>()?;
        tables.push(Table::from(table));
    }
    Ok((numbers, tables))
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n
22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19\n
 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6\n
14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part1() {
        let (numbers, tables) = parse(TEST_INPUT).unwrap();
        assert_eq!(part1(numbers, tables), 4512);
    }
    #[test]
    fn test_part2() {
        let (numbers, tables) = parse(TEST_INPUT).unwrap();
        assert_eq!(part2(numbers, tables), 1924);
    }
}
