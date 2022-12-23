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

fn display_elf(pos: &(u32, u32), facing: Facing) {
    let c = match facing {
        Facing::North => '^',
        Facing::East => '>',
        Facing::West => '<',
        Facing::South => '^',
    };

    println!("Elf Position: {:?}\nElf Facing: {}", pos, c);
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

fn run_part1(map: &Map, cmds: &Vec<Cmd>) -> u32 {
    println!("{:?}", map);
    let pos: (u32, u32) = (0, 0);
    let facing = Facing::East;

    println!();
    cmds.iter()
        .for_each(|c| println!("{:?}", c));
    println!();

    display_elf(&pos, facing);
    println!();

    return 0;
}

//fn run_part2() -> usize {
//    return 0;
//}
