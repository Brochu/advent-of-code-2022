use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct ElfPair {
    first: (u8, u8),
    second: (u8, u8),
}

impl FromStr for ElfPair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves = s.split(",")
            .map(|e| {
                let (f, s) = e.split_once("-").unwrap();

                let start = f.parse::<u8>().unwrap();
                let end = s.parse::<u8>().unwrap();

                (start, end)
            })
            .collect::<Vec<(u8, u8)>>();

        return Ok(ElfPair{ first: (elves[0].0, elves[0].1), second: (elves[1].0, elves[1].1) });
    }
}

impl ElfPair {
    fn has_full_overlaps(&self) -> bool {
        return 
            (self.first.0 >= self.second.0 && self.first.1 <= self.second.1) ||
            (self.second.0 >= self.first.0 && self.second.1 <= self.first.1);
    }

    fn has_any_overlaps(&self) -> bool {
        let mut f_range = self.first.0..=self.first.1;
        let mut s_range = self.second.0..=self.second.1;

        return f_range.any(|e| s_range.contains(&e)) || s_range.any(|e| f_range.contains(&e));
    }
}

pub fn main() {
    println!("[Day4] Solutions:");

    let pairs = include_str!("../data/day4.input")
        .lines()
        .map(|line| ElfPair::from_str(line).unwrap())
        .collect::<Vec<ElfPair>>();

    println!("[Day4] Part 1 => {}", run_part1(&pairs));
    println!("[Day4] Part 2 => {}", run_part2(&pairs));

    println!("[Day4] Complete -----------------------");
}

fn run_part1(pairs: &Vec<ElfPair>) -> u64 {
    return pairs.iter()
        .filter(|p| p.has_full_overlaps())
        .count() as u64;
}

fn run_part2(pairs: &Vec<ElfPair>) -> u64 {
    return pairs.iter()
        .filter(|p| p.has_any_overlaps())
        .count() as u64;
}
