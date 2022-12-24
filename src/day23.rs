type Elf = (i32, i32);

pub fn main() {
    println!("[Day23] Solutions:");

    let elves = include_str!("../data/day23.example.small").lines().enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate()
                .filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((x as i32, y as i32))
                    }
                    else {
                        None
                    }
                })
        })
        .collect::<Vec<Elf>>();

    println!("[Day23] Part 1 => {}", run_part1(&elves));
    //println!("[Day23] Part 2 => {}", run_part2());

    println!("[Day23] Complete -----------------------");
}

fn run_part1(elves: &Vec<Elf>) -> i64 {
    elves.iter()
        .for_each(|e| println!("{:?}", e));

    return elves.len() as i64;
}

//fn run_part2() -> usize {
//    return 0;
//}
