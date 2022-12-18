#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum Shape {
    HLine,
    Cross,
    Corner,
    VLine,
    Block,
}

pub fn main() {
    println!("[Day17] Solutions:");

    let push_dirs = include_str!("../data/day17.example")
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| {
                match c {
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!("Could not parse direction"),
                }
            })
        })
        .collect::<Vec<Direction>>();

    let shapes: Vec<Shape> = vec![Shape::HLine, Shape::Cross, Shape::Corner, Shape::VLine, Shape::Block];

    println!("[Day17] Part 1 => {}", run_part1(&push_dirs, &shapes));
    //println!("[Day17] Part 2 => {}", run_part2());

    println!("[Day17] Complete -----------------------");
}

fn run_part1(dirs: &Vec<Direction>, shapes: &Vec<Shape>) -> u32 {
    println!("{:?}", dirs);
    let mut dir_cycle = dirs[0..5].iter().cycle();
    let mut shape_cycle = shapes.iter().cycle();

    for i in 0..15 {
        println!("[{i}] : {:?}", dir_cycle.next().unwrap());
    }
    println!();
    for i in 0..15 {
        println!("[{i}] : {:?}", shape_cycle.next().unwrap());
    }

    return 0;
}

//fn run_part2() -> usize {
//    return 0;
//}
