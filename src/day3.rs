pub fn main() {
    println!("[Day3] Solutions:");

    let part1 = run_part1();
    println!("[Day3] Part 1 => {}", part1);

    let part2 = run_part2();
    println!("[Day3] Part 2 => {}", part2);

    println!("[Day3] Complete -----------------------");
}

fn run_part1() -> u64 {
    let lines: Vec<_> = include_str!("../data/day3.example")
        .lines()
        .collect();

    return lines.len() as u64;
}

fn run_part2() -> u64 {
    let lines: Vec<_> = include_str!("../data/day3.example")
        .lines()
        .collect();

    return lines.len() as u64;
}
