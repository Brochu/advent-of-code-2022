pub fn main() {
    println!("[Day1] Solutions:");

    let part1 = run_part1();
    println!("[Day1] Part 1 => {}", part1);

    let part2 = run_part2();
    println!("[Day1] Part 2 => {}", part2);

    println!("[Day1] Complete -----------------------");
}

fn run_part1() -> u64 {
    let lines: Vec<_> = include_str!("../data/day1.example")
        .lines()
        .collect();

    return lines.len() as u64;
}

fn run_part2() -> u64 {
    let lines: Vec<_> = include_str!("../data/day1.example")
        .lines()
        .collect();

    return lines.len() as u64;
}
