use std::fmt::Display;

type Blizzard = ((u32, u32), u32);

struct Map {
    width: u32,
    height: u32,

    blizzards: Vec<Blizzard>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let blizz = self.blizzards.iter().fold(String::new(), |output, ((x, y), cycle)| {
            format!("{} (({}, {}), {})", output, x, y, cycle)
        });

        write!(f, "Map ({} x {})\nBlizzards: {}", self.width, self.height, blizz)
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
                '^' | 'v' => Some(((x as u32, y as u32), height)),
                '<' | '>' => Some(((x as u32, y as u32), width)),
                _ => None,
            }
        }))
        .collect();

    let map = Map { width, height, blizzards };

    println!("[Day24] Part 1 => {}", run_part1(&map));
    //println!("[Day24] Part 2 => {}", run_part2());

    println!("[Day24] Complete -----------------------");
}

fn run_part1(map: &Map) -> u32 {
    println!("{}", map);
    return 0;
}

//fn run_part2() -> usize {
//    return 0;
//}
