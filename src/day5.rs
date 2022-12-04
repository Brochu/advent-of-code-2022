pub fn main() {
    println!("[Day5] Solutions:");

    let lines = include_str!("../data/day5.example")
        .lines()
        .collect::<Vec<&str>>();

    println!("[Day5] Part 1 => {}", run_part1(&lines));
    println!("[Day5] Part 2 => {}", run_part2(&lines));

    println!("[Day5] Complete -----------------------");
}

fn run_part1(lines: &Vec<&str>) -> u64 {
    return lines.len() as u64;
}

fn run_part2(lines: &Vec<&str>) -> u64 {
    return lines.len() as u64;
}
