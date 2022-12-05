pub fn main() {
    println!("[Day6] Solutions:");

    let lines = include_str!("../data/day6.example")
        .lines()
        .collect::<Vec<&str>>();

    println!("[Day6] Part 1 => {}", run_part1(&lines));
    println!("[Day6] Part 2 => {}", run_part2(&lines));

    println!("[Day6] Complete -----------------------");
}

fn run_part1(lines: &Vec<&str>) -> u64 {
    return lines.len() as u64;
}

fn run_part2(lines: &Vec<&str>) -> u64 {
    return lines.len() as u64;
}
