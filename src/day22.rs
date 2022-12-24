use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum Cmd {
    Forward(i32),
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

type Map = HashMap<(i32, i32), Tile>;

fn find_start(map: &Map) -> (i32, i32) {
    return map.iter()
        .filter(|(&(_, y), _)| y == 0)
        .fold((i32::MAX, 0), |mut start, (&(x, _), _)| {
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
                        '.' => Some(((x as i32, y as i32), Tile::Air)),
                        '#' => Some(((x as i32, y as i32), Tile::Wall)),
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
                cmds.push(Cmd::Forward(num_str.parse::<i32>().unwrap()));
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
        cmds.push(Cmd::Forward(num_str.parse::<i32>().unwrap()));
    }

    return cmds;
}

pub fn main() {
    println!("[Day22] Solutions:");

    let (map_str, cmds_str) = include_str!("../data/day22.input").split_once("\r\n\r\n").unwrap();
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

fn find_next_tile(map: &Map, pos: &(i32, i32), facing: &Facing) -> ((i32, i32), Tile) {
    let &(curr_x, curr_y) = pos;
    let next_pos = match facing {
        Facing::North => (curr_x, curr_y - 1),
        Facing::East => (curr_x + 1, curr_y),
        Facing::West => (curr_x - 1, curr_y),
        Facing::South => (curr_x, curr_y + 1),
    };

    match map.get(&next_pos) {
        Some(&tile) => {
            (next_pos, tile)
        },
        None => {
            match *facing {
                Facing::North => {
                    let mut y = 0;
                    let mut t = Tile::Air;
                    for c_y in 0..200 {
                        if let Some(&tile) = map.get(&(next_pos.0, c_y)) {
                            y = c_y;
                            t = tile;
                        }
                    }
                    ((next_pos.0, y), t)
                },
                Facing::East => {
                    let mut x = 0;
                    let mut t = Tile::Air;
                    for c_x in 0..next_pos.0 {
                        if let Some(&tile) = map.get(&(c_x, next_pos.1)) {
                            x = c_x;
                            t = tile;
                            break;
                        }
                    }
                    ((x, next_pos.1), t)
                },
                Facing::West => {
                    let mut x = 0;
                    let mut t = Tile::Air;
                    for c_x in 0..150 {
                        if let Some(&tile) = map.get(&(c_x, next_pos.1)) {
                            x = c_x;
                            t = tile;
                        }
                    }
                    ((x, next_pos.1), t)
                },
                Facing::South => {
                    let mut y = 0;
                    let mut t = Tile::Air;
                    for c_y in 0..next_pos.1 {
                        if let Some(&tile) = map.get(&(next_pos.0, c_y)) {
                            y = c_y;
                            t = tile;
                            break;
                        }
                    }
                    ((next_pos.0, y), t)
                },
            }
        },
    }
}

fn apply_command(map: &Map, pos: &mut (i32, i32), facing: &mut Facing, cmd: &Cmd) {
    //println!("Applying cmd: {:?}; from: {:?}, {:?}", cmd, pos, facing);

    if let &Cmd::Forward(dist) = cmd {
        //println!("Move elf forward for {} steps", dist);

        for _ in 0..dist {
            let (next_pos, next_tile) = find_next_tile(map, pos, facing);

            match next_tile {
                Tile::Air => {
                    //println!("Next pos: {:?}", next_pos);
                    *pos = next_pos;
                },
                Tile::Wall => {
                    break; // Hit a wall, start next cmd
                }
            };
        }
    }
    else {
        *facing = turn_elf(*facing, *cmd);
    }
}

fn run_part1(map: &Map, cmds: &Vec<Cmd>) -> i32 {
    let pos = find_start(map);
    let facing = Facing::East;
    //println!("\nStarting:\n\tpos: {:?}\n\tface: {:?}", pos, facing);
    //println!();

    let ((column, row), end_facing) = cmds.iter()
        .fold((pos, facing), |(mut  p, mut f), cmd| {
            apply_command(map, &mut p, &mut f, cmd);
            //println!("Pos: {:?}; Facing: {:?}", p, f);
            //println!();

            (p, f)
        });

    let password = ((row+1) * 1000) + ((column+1) * 4) + match end_facing {
        Facing::North => 3,
        Facing::East => 0,
        Facing::West => 2,
        Facing::South => 1,
    };
    return password;
}

//fn run_part2() -> usize {
//    return 0;
//}
