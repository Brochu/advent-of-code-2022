use std::collections::HashSet;

pub fn main() {
    println!("[Day3] Solutions:");

    let rucksacks: Vec<&str> = include_str!("../data/day3.input")
        .lines()
        .collect();

    let part1 = run_part1(&rucksacks);
    println!("[Day3] Part 1 => {}", part1);

    let part2 = run_part2(&rucksacks);
    println!("[Day3] Part 2 => {}", part2);

    println!("[Day3] Complete -----------------------");
}

fn calc_priority(item: &char) -> u64 {
    if item.is_uppercase() {
        let base = (b'A' as u8) - 1;
        return ((*item as u8) - base + 26) as u64;
    }
    else {
        let base = (b'a' as u8) - 1;
        return ((*item as u8) - base) as u64;
    }
}

fn run_part1(sacks: &Vec<&str>) -> u64 {
    let sets: Vec<_> = sacks.iter()
        .map(|list| {
            let first: HashSet<char> = list.chars().take(list.len() / 2).collect();
            let second: HashSet<char> = list.chars().skip(list.len() / 2).take(list.len() / 2).collect();
            (first, second)
        })
        .collect();

    return sets.iter()
        .flat_map(|set| set.0.intersection(&set.1))
        .map(|r| calc_priority(r))
        .sum();
}

fn run_part2(sacks: &Vec<&str>) -> u64 {
    let groups: Vec<_> = sacks[0..]
        .chunks(3)
        .map(|g| {
            let letters: Vec<HashSet<char>> = g.iter()
                .map(|s| s.chars().collect()).collect();

            letters
        })
        .collect();

    let bands: Vec<_> = groups.iter()
        .map(|g| {
            g
        })
        .collect();

    let mut badges = Vec::<char>::new();
    for b in bands {
        let first: HashSet<&char> = b[0].intersection(&b[1]).collect();
        let second: HashSet<&char> = b[0].intersection(&b[2]).collect();

        let test = first.intersection(&second);
        test.for_each(|t| badges.push(**t));
    }

    return badges.iter()
        .map(|c| calc_priority(c))
        .sum();
}
