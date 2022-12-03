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
    return match item {
        'A'..='Z' => (*item as u64) - 38,
        'a'..='z' => (*item as u64) - 96,
        _ => 0,
    };
}

type Comp = HashSet<char>;

fn run_part1(sacks: &Vec<&str>) -> u64 {
    let compartments = sacks.iter()
        .map(|list| {
            let (left, right) = list.split_at(list.len() / 2);
            (left.chars().collect::<Comp>(), right.chars().collect::<Comp>())
        })
        .collect::<Vec<(Comp, Comp)>>();

    return compartments.iter()
        .flat_map(|c| c.0.intersection(&c.1))
        .map(|ch| calc_priority(ch))
        .sum::<u64>();
}

fn run_part2(sacks: &Vec<&str>) -> u64 {
    return sacks[..].chunks(3)
        .map(|group| {
            group.iter()
                .fold(String::new(), |items, elf| {
                    if items.len() > 0 {
                        //println!("itm : {:?}", items);
                        //println!("elf : {:?}", elf);

                        let past = items.chars().collect::<Comp>();
                        let curr = elf.chars().collect::<Comp>();
                        //println!("itm : {:?}", past);
                        //println!("elf : {:?}", curr);

                        curr.intersection(&past).map(|&c| c).collect::<String>()
                    }
                    else {
                        let curr = elf.chars().collect::<Comp>();
                        curr.iter().collect::<String>()
                    }
                })
        })
        .map(|c| calc_priority(&c.chars().next().unwrap()))
        .sum();
}
