use std::ops::RangeInclusive;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(Password::parse)
        .map(Result::unwrap)
        .filter(|password| password.is_valid_part_1())
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(Password::parse)
        .map(Result::unwrap)
        .filter(|password| password.is_valid_part_2())
        .count()
}

#[derive(Debug, PartialEq)]
pub struct Password<'a> {
    range: RangeInclusive<usize>,
    byte: u8,
    text: &'a str,
}

impl<'a> Password<'a> {
    fn parse(input: &'a str) -> anyhow::Result<Self> {
        peg::parser! {
            grammar parser() for str {
                rule number() -> usize
                = n:$(['0'..='9']+) {n.parse().unwrap()}

                rule range() -> RangeInclusive<usize>
                = min:number() "-" max:number() {min..=max}

                rule byte() -> u8
                = letter:$(['a'..='z']) { letter.as_bytes()[0] }

                rule text() -> &'input str
                = letters:$([_]*) { letters }

                pub(crate) rule root() -> Password<'input>
                = range:range() " " byte:byte() ": " text:text() {
                Password { range, byte, text}
                }
            }
        }
        Ok(parser::root(input).unwrap())
    }

    fn is_valid_part_1(&self) -> bool {
        self.range.contains(
            &self
                .text
                .as_bytes()
                .iter()
                .copied()
                .filter(|&b| b == self.byte)
                .count(),
        )
    }

    fn is_valid_part_2(&self) -> bool {
        let temp = self.range.clone();
        let position = vec![temp.start() - 1, temp.end() - 1];
        position
            .iter()
            .filter(|&&index| self.text.as_bytes()[index] == self.byte)
            .count()
            == 1
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    #[test]
    fn test_is_valid_part_1_false() {
        let pp = Password {
            range: 1..=3,
            byte: b'a',
            text: "aaaah",
        };
        assert_eq!(pp.is_valid_part_1(), false, "too many 'a's");
    }

    #[test]
    fn test_is_valid_part_1_true() {
        let pp = Password {
            range: 1..=3,
            byte: b'a',
            text: "hades",
        };
        assert_eq!(pp.is_valid_part_1(), true);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Password::parse("1-3 a: banana").unwrap(),
            Password {
                range: 1..=3,
                byte: b'a',
                text: "banana",
            }
        );
    }
}
