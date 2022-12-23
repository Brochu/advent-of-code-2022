use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum Cmd {
    Forward(u32),
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Air,
    Wall,
}

#[derive(Copy, Clone, Debug)]
enum Facing {
    North,
    East,
    West,
    South,
}

type Map = HashMap<(u32, u32), Tile>;

fn find_start(map: &Map) -> (u32, u32) {
    return map.iter()
        .filter(|(&(_, y), _)| y == 0)
        .fold((u32::MAX, 0), |mut start, (&(x, _), _)| {
            if x < start.0 {
                start.0 = x;
            }
            start
        });
}

fn build_map(map_str: &str) -> Map {
    return map_str.lines().enumerate()
        .flat_map(|(y, line)| {

            line.chars().enumerate()
                .filter_map(move |(x, c)| {

                    match c {
                        '.' => Some(((x as u32, y as u32), Tile::Air)),
                        '#' => Some(((x as u32, y as u32), Tile::Wall)),
                        _ => None,
                    }
                })
        })
    .fold(Map::new(), |mut map, (coord, tile)| {
        map.insert(coord, tile);
        map
    });
}

fn build_cmds(cmds_str: &str) -> Vec<Cmd> {
    let (mut cmds, num_str) = cmds_str.trim().chars()
        .fold((Vec::new(), String::from("")), |(mut cmds, mut num_str), c| {
            if c.is_digit(10) {
                num_str.push(c);
            }
            else {
                cmds.push(Cmd::Forward(num_str.parse::<u32>().unwrap()));
                num_str.clear();

                match c {
                    'L' => cmds.push(Cmd::Left),
                    'R' => cmds.push(Cmd::Right),
                    _ => panic!("Invalid direction"),
                };
            }
            (cmds, num_str)
        });

    if num_str.len() > 0 {
        cmds.push(Cmd::Forward(num_str.parse::<u32>().unwrap()));
    }

    return cmds;
}

pub fn main() {
    println!("[Day22] Solutions:");

    let (map_str, cmds_str) = include_str!("../data/day22.example").split_once("\r\n\r\n").unwrap();
    let map = build_map(map_str);
    let cmds = build_cmds(cmds_str);

    println!("[Day22] Part 1 => {}", run_part1(&map, &cmds));
    //println!("[Day22] Part 2 => {}", run_part2());

    println!("[Day22] Complete -----------------------");
}

fn turn_elf(facing: Facing, cmd: Cmd) -> Facing {
    return match facing {
        Facing::North => {
            match cmd {
                Cmd::Left => Facing::West,
                Cmd::Right => Facing::East,
                Cmd::Forward(_) => facing,
            }
        },
        Facing::East => {
            match cmd {
                Cmd::Left => Facing::North,
                Cmd::Right => Facing::South,
                Cmd::Forward(_) => facing,
            }
        },
        Facing::West => {
            match cmd {
                Cmd::Left => Facing::South,
                Cmd::Right => Facing::North,
                Cmd::Forward(_) => facing,
            }
        },
        Facing::South => {
            match cmd {
                Cmd::Left => Facing::East,
                Cmd::Right => Facing::West,
                Cmd::Forward(_) => facing,
            }
        },
    }
}

fn find_next_tile(map: &Map, pos: &(u32, u32), facing: &Facing) -> ((u32, u32), Tile) {
    let &(curr_x, curr_y) = pos;
    let next_pos = match facing {
        Facing::North => (curr_x, curr_y + 1),
        Facing::East => (curr_x + 1, curr_y),
        Facing::West => (curr_x - 1, curr_y),
        Facing::South => (curr_x, curr_y - 1),
    };

    match map.get(&next_pos) {
        Some(&tile) => {
            (next_pos, tile)
        },
        None => {
            // Need to wrap the position around based on facing
            (next_pos, Tile::Air)
        },
    }
}

fn apply_command(map: &Map, pos: &mut (u32, u32), facing: &mut Facing, cmd: &Cmd) {
    println!("Applying cmd: {:?}; from: {:?}, {:?}", cmd, pos, facing);

    if let &Cmd::Forward(dist) = cmd {
        println!("Move elf forward for {} steps", dist);

        for _ in 0..dist {
            let (next_pos, next_tile) = find_next_tile(map, pos, facing);

            match next_tile {
                Tile::Air => {
                    println!("Next pos: {:?}", next_pos);
                    // Update pos
                },
                Tile::Wall => {
                    // Hit a wall, start next cmd
                    break;
                }
            };
        }
    }
    else {
        *facing = turn_elf(*facing, *cmd);
    }
}

fn run_part1(map: &Map, cmds: &Vec<Cmd>) -> u32 {
    let pos = find_start(map);
    let facing = Facing::East;
    println!("\nStarting:\n\tpos: {:?}\n\tface: {:?}", pos, facing);
    println!();

    let (_end_pos, _end_facing) = cmds[0..1].iter()
        .fold((pos, facing), |(mut  p, mut f), cmd| {
            apply_command(map, &mut p, &mut f, cmd);
            println!("Pos: {:?}; Facing: {:?}", p, f);
            println!();

            (p, f)
        });

    return 0;
}

//fn run_part2() -> usize {
//    return 0;
//}
