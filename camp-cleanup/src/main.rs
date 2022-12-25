use std::{
    io::{stdin, Read},
    num::ParseIntError,
    str::FromStr,
};

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");
    let ans: usize = input
        .split("\n")
        .map(|s| {
            if s.is_empty() {
                return 0;
            }
            let (range1, range2) = s.split_once(",").unwrap();
            let (range1, range2) = (
                range1.parse::<Range>().unwrap(),
                range2.parse::<Range>().unwrap(),
            );
            // if range1.contains(&range2) || range2.contains(&range1) {
            //     1
            // } else {
            //     0
            // }
            if range1.overlap(&range2) {
                1
            } else {
                0
            }
        })
        .sum();
    println!("{ans}");
}

struct Range(usize, usize);

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once("-").unwrap();
        let l_from_str = l.parse::<usize>()?;
        let r_from_str = r.parse::<usize>()?;
        Ok(Range(l_from_str, r_from_str))
    }
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlap(&self, other: &Range) -> bool {
        !(self.1 < other.0 || self.0 > other.1)
    }
}
