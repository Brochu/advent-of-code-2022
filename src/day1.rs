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
    let mut totals = include_str!("../data/day1.input")
        .split("\r\n\r\n")
        .map(|e| {
            e.lines()
                .map(|v| v.trim().parse::<u64>().unwrap())
                .sum()
        })
        .collect::<Vec<u64>>();
    totals.sort();

    return totals.iter()
        .rev()
        .take(3)
        .sum();
}
