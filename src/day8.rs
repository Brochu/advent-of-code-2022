pub fn main() {
    println!("[Day8] Solutions:");

    let lines = include_str!("../data/day8.example")
        .lines()
        .collect::<Vec<&str>>();

    println!("[Day8] Part 1 => {}", run_part1(&lines));
    println!("[Day8] Part 2 => {}", run_part2(&lines));

    println!("[Day8] Complete -----------------------");
}

fn run_part1(lines: &Vec<&str>) -> usize {
    return lines.len();
}

fn run_part2(lines: &Vec<&str>) -> usize {
    return lines.len();
}
