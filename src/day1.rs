pub fn main() {
    println!("[Day1] Solutions:");

    let part1 = run_part1();
    println!("[Day1] Part 1 => {}", part1);

    let part2 = run_part2();
    println!("[Day1] Part 2 => {}", part2);

    println!("[Day1] Complete -----------------------");
}

fn run_part1() -> u64 {
    return include_str!("../data/day1.input")
        .split("\r\n\r\n")
        .map(|e| {
            e.lines()
                .map(|v| v.trim().parse::<u64>().unwrap())
                .sum()
        })
        .max().unwrap();
}

fn run_part2() -> u64 {
    let mut totals: Vec<u64> = include_str!("../data/day1.input")
        .split("\r\n\r\n")
        .map(|e| {
            e.lines()
                .map(|v| v.trim().parse::<u64>().unwrap())
                .sum()
        })
        .collect();

    //println!("{:?}", totals);
    totals.sort();
    totals.reverse();
    //println!("{:?}", totals);

    let top = &totals[0..3];
    //println!("{:?}", top);

    return top.iter().sum();
}
