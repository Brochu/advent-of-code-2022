pub fn main() {
    println!("[Day15] Solutions:");

    let lines = include_str!("../data/day15.example")
        .lines()
        .collect::<Vec<&str>>();

    println!("[Day15] Part 1 => {}", run_part1(&lines));
    println!("[Day15] Part 2 => {}", run_part2(&lines));

    println!("[Day15] Complete -----------------------");
}

fn run_part1(lines: &Vec<&str>) -> usize {
    return lines.len();
}

fn run_part2(lines: &Vec<&str>) -> usize {
    return lines.len();
}
