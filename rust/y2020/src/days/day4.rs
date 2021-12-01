fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Year(u32);

#[derive(Clone, Copy, PartialEq, Debug)]
enum Height {
    Cm(u32),
    In(u32),
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Color<'a>(&'a str);

#[derive(Clone, Copy, PartialEq, Debug)]
struct ID<'a>(&'a str);

#[derive(PartialEq, Debug)]
struct Passport<'a> {
    byr: Year,
    iyr: Year,
    eyr: Year,
    hgt: Height,
    hcl: Color<'a>,
    ecl: Color<'a>,
    pid: ID<'a>,
    cid: Option<ID<'a>>,
}

#[derive(PartialEq, Debug, Default)]
struct PassportBuilder<'a> {
    byr: Option<Year>,
    iyr: Option<Year>,
    eyr: Option<Year>,
    hgt: Option<Height>,
    hcl: Option<Color<'a>>,
    ecl: Option<Color<'a>>,
    pid: Option<ID<'a>>,
    cid: Option<ID<'a>>,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("missing field: {0}")]
    MissingField(&'static str),
}

impl<'a> PassportBuilder<'a> {
    fn new(self) -> Result<Passport<'a>, Error> {
        Ok(Passport {
            byr: self.byr.ok_or(Error::MissingField("birth year"))?,
            iyr: self.iyr.ok_or(Error::MissingField("issue year"))?,
            eyr: self.eyr.ok_or(Error::MissingField("expiration year"))?,
            hgt: self.hgt.ok_or(Error::MissingField("height"))?,
            hcl: self.hcl.ok_or(Error::MissingField("hair color"))?,
            ecl: self.ecl.ok_or(Error::MissingField("eye color"))?,
            pid: self.pid.ok_or(Error::MissingField("passport id"))?,
            cid: self.cid,
        })
    }
}
