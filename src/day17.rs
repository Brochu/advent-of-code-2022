use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

type Shape = Vec<(i32, i32)>;

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
    println!("[Day17] Part 2 => {}", run_part2());

    println!("[Day17] Complete -----------------------");
}

fn _show_map(status: &HashSet<(i32, i32)>, rock: &Shape, height: i32, width: i32) {
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

fn update_rock_pos(status: &HashSet<(i32, i32)>,
                 rock: &mut Shape,
                 dir: &Direction,
                 width: i32) -> bool {

    // Check for collide with walls 0 and width
    let gas_move = match dir {
        Direction::Left => {
            let sides = rock.iter().any(|&(pos_x, _)| pos_x == 0);
            let rock = sides || rock.iter().any(|&(pos_x, pos_y)| {
                status.get(&(pos_x-1, pos_y)).is_some()
            });
            if sides || rock { 0 } else { -1 }
        },
        Direction::Right => {
            let sides = rock.iter().any(|&(pos_x, _)| pos_x == width - 1);
            let rock = sides || rock.iter().any(|&(pos_x, pos_y)| {
                status.get(&(pos_x+1, pos_y)).is_some()
            });
            if sides || rock { 0 } else { 1 }
        },
    };

    // Move rock based on gas movement
    rock.iter_mut().for_each(|(pos_x, _)| { *pos_x += gas_move; });
    
    //TODO Rock falls
    //  Check for collide with bottom + other rocks
    //  Add rock to status when collision happens
    let reach_bottom = rock.iter().any(|&(_, pos_y)| pos_y == 0);
    let reach_rock = reach_bottom || rock.iter().any(|&(pos_x, pos_y)| {
        status.get(&(pos_x, pos_y-1)).is_some()
    });

    if reach_bottom || reach_rock {
        return false;
    }
    else {
        rock.iter_mut().for_each(|(_, pos_y)| { *pos_y -= 1; });
        return true;
    }
}

fn run_part1(dirs: &Vec<Direction>, shapes: &Vec<Shape>) -> i32 {
    //println!("{:?}", dirs);
    //println!("{:?}", shapes);
    let mut dir_cycle = dirs.iter().cycle();
    let mut shape_cycle = shapes.iter().cycle();

    //for i in 0..15 {
    //    println!("[{i}] : {:?}", dir_cycle.next().unwrap());
    //}
    //println!();
    //for i in 0..15 {
    //    println!("[{i}] : {:?}", shape_cycle.next().unwrap());
    //}

    const WIDTH: i32 = 7;
    let mut height: i32 = 0;
    let mut status = HashSet::<(i32, i32)>::new();

    for _ in 0..2022 {
        let mut rock = shape_cycle.next().unwrap().clone()
            .iter()
            .map(|&(pos_x, pos_y)| (pos_x, pos_y + height))
            .collect::<Vec<(i32, i32)>>();
        //show_map(&status, &rock, height+8, WIDTH);

        while update_rock_pos(&status, &mut rock, dir_cycle.next().unwrap(), WIDTH) {
            //show_map(&status, &rock, height+8, WIDTH);
        }

        rock.iter().for_each(|&p| {
            status.insert(p);
        });
        height = status.iter().map(|&(_, pos_y)| pos_y).max().unwrap() + 1;

        //show_map(&status, &rock, height+8, WIDTH);
    }

    return height;
}

fn run_part2() -> usize {
    //TODO: In order to solve part 2, need to find a cycle that repeats and adds a given height
    //  Find out how many cycles are left, and calculate the amount of height gained instead of full sim
    return 0;
}
