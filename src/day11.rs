pub fn main() {
    println!("[Day11] Solutions:");

    let lines = include_str!("../data/day11.example")
        .lines()
        .collect::<Vec<&str>>();

    println!("[Day11] Part 1 => {}", run_part1(&lines));
    println!("[Day11] Part 2 => {}", run_part2(&lines));

    println!("[Day11] Complete -----------------------");
}

fn run_part1(lines: &Vec<&str>) -> usize {
    return lines.len();
}

fn run_part2(lines: &Vec<&str>) -> usize {
    return lines.len();
}
