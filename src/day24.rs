use std::fmt::Display;

enum Direction {
    Horizontal,
    Vertical,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Horizontal => write!(f, "H"),
            Direction::Vertical => write!(f, "V"),
        }
    }
}

type Blizzard = (u32, Direction); // Starting Pos, Direction

struct Map {
    width: u32,
    height: u32,

    blizzards: Vec<Blizzard>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let blizz = self.blizzards.iter().fold(String::new(), |output, (start, dir)| {
            format!("{} ({}, {})", output, start, dir)
        });

        write!(f, "Map ({} x {})\nBlizzards: {}", self.width, self.height, blizz)
    }
}
impl Map {
    fn get_tile(&self, time: u32, x: u32, y: u32) -> char {
        if y == 0 || x == 0 || y == self.height + 1 || x == self.width + 1 {
            return '#';
        }
        else {
            println!("Find if there are any blizzards at ({}, {}) for time = {}", x, y, time);
            return '.';
        }
    }
}

const DATA_STR: &str = include_str!("../data/day24.example.small");

pub fn main() {
    println!("[Day24] Solutions:");

    let width = DATA_STR.lines().next().unwrap().len() as u32 - 2;
    let height = DATA_STR.lines().count() as u32 - 2;
    let blizzards = DATA_STR
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().filter_map(move |(x, c)| {
            match c {
                '^' | 'v' => Some((x as u32, Direction::Vertical)),
                '<' | '>' => Some((y as u32, Direction::Horizontal)),
                _ => None,
            }
        }))
        .collect();

    let map = Map { width, height, blizzards };

    println!("[Day24] Part 1 => {}", run_part1(&map));
    //println!("[Day24] Part 2 => {}", run_part2());

    println!("[Day24] Complete -----------------------");
}

fn show_map(map: &Map, time: u32) {
    for y in 0..map.height + 2 {
        for x in 0..map.width + 2 {
            print!(" {} ", map.get_tile(time, x, y));
        }
        println!();
    }
}

fn run_part1(map: &Map) -> u32 {
    println!("{}", map);
    println!();
    show_map(map, 0);

    return 0;
}

//fn run_part2() -> usize {
//    return 0;
//}
