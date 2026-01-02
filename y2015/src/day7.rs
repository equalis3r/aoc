use std::collections::HashMap;
const INPUT: &str = include_str!("./assets/day7.txt");

#[derive(Debug)]
pub enum Logic<'a> {
    ASSIGN([Signal<'a>; 2]),
    NOT([Signal<'a>; 2]),
    OR([Signal<'a>; 3]),
    AND([Signal<'a>; 3]),
    RSHIFT([Signal<'a>; 3]),
    LSHIFT([Signal<'a>; 3]),
}

#[derive(Debug)]
pub enum Signal<'a> {
    Value(u16),
    Name(&'a str),
}

impl<'a> Logic<'a> {
    pub fn execute(&self, table: &mut HashMap<&str, u16>) {
        match self {
            Logic::ASSIGN([s1, s2]) => match s1 {
                Signal::Value(val) => {
                    table.insert(s2, *val);
                }
                Signal::Name(name) => {}
            },
            Logic::NOT([s1, s2]) => {}
            Logic::OR([s1, s2, s3]) => {}
            Logic::AND([s1, s2, s3]) => {}
            Logic::RSHIFT([s1, s2, s3]) => {}
            Logic::LSHIFT([s1, s2, s3]) => {}
        }
    }
    pub fn parse_line(input: &str) -> Logic {
        let components: Vec<&str> = input.split(' ').collect();
        if input.contains("NOT") {
            Logic::NOT([Signal::parse(components[1]), Signal::parse(components[3])])
        } else if input.contains("OR") {
            Logic::OR([
                Signal::parse(components[0]),
                Signal::parse(components[2]),
                Signal::parse(components[4]),
            ])
        } else if input.contains("AND") {
            Logic::AND([
                Signal::parse(components[0]),
                Signal::parse(components[2]),
                Signal::parse(components[4]),
            ])
        } else if input.contains("RSHIFT") {
            Logic::LSHIFT([
                Signal::parse(components[0]),
                Signal::parse(components[2]),
                Signal::parse(components[4]),
            ])
        } else if input.contains("LSHIFT") {
            Logic::RSHIFT([
                Signal::parse(components[0]),
                Signal::parse(components[2]),
                Signal::parse(components[4]),
            ])
        } else {
            Logic::ASSIGN([Signal::parse(components[0]), Signal::parse(components[2])])
        }
    }

    pub fn parse(input: &str) -> Vec<Logic> {
        input.lines().map(Logic::parse_line).collect::<Vec<_>>()
    }
}

impl<'a> Signal<'a> {
    pub fn parse(input: &str) -> Signal {
        let res = input.parse::<u16>();
        match res {
            Ok(val) => Signal::Value(val),
            Err(_) => Signal::Name(input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
}
