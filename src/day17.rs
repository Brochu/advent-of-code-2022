use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

type Shape = Vec<(u32, u32)>;

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

    let shapes: Vec<Shape> = vec![
        vec![(2, 3), (3, 3), (4, 3), (5, 3)],
        vec![(2, 4), (3, 4), (4, 4), (3, 5), (3, 3)],
        vec![(2, 3), (3, 3), (4, 3), (4, 4), (4, 5)],
        vec![(2, 3), (2, 4), (2, 5), (2, 6)],
        vec![(2, 3), (3, 3), (2, 4), (3, 4)],
    ];

    println!("[Day17] Part 1 => {}", run_part1(&push_dirs, &shapes));
    //println!("[Day17] Part 2 => {}", run_part2());

    println!("[Day17] Complete -----------------------");
}

fn show_map(status: &HashSet<(u32, u32)>, rock: &Shape, height: u32, width: u32) {
    println!();

    for y in (0..height).rev() {
        for x in 0..width {
            if status.contains(&(x, y)) { print!("#"); }
            else if rock.iter().any(|&p| p == (x, y)) { print!("@"); }
            else { print!("."); }
        }
        println!();
    }
    println!();
}

fn run_part1(dirs: &Vec<Direction>, shapes: &Vec<Shape>) -> u32 {
    //println!("{:?}", dirs);
    //println!("{:?}", shapes);
    //let mut dir_cycle = dirs[0..5].iter().cycle();
    let mut shape_cycle = shapes.iter().cycle();

    //for i in 0..15 {
    //    println!("[{i}] : {:?}", dir_cycle.next().unwrap());
    //}
    //println!();
    //for i in 0..15 {
    //    println!("[{i}] : {:?}", shape_cycle.next().unwrap());
    //}

    const WIDTH: u32 = 7;
    let mut height: u32 = 8;
    let mut status = HashSet::<(u32, u32)>::new();

    for _ in 0..1 {
        let mut rock = shape_cycle.next().unwrap().clone();
        show_map(&status, &rock, height, WIDTH);

        //TODO Apply direction push
        //  Check for collide with walls 0 and 7
        //TODO Rock falls
        //  Check for collide with bottom + other rocks
        //  Add rock to status when collision happens
    }

    return 0;
}

//fn run_part2() -> usize {
//    return 0;
//}
