use std::collections::HashSet;
use std::fmt::Display;

type Pos = (i32, i32);
type Blizzard = (Pos, char); // Starting Pos, Direction char

struct Map {
    width: i32,
    height: i32,

    start: Pos,
    end: Pos,

    blizzards: Vec<Blizzard>,
}

impl Map {
    fn blizz_for_time(&self, time: i32) -> HashSet<Pos> {
        self.blizzards.iter()
            .fold(HashSet::new(), |mut lut, ((x, y), dir)| {
                let new_pos = match dir {
                    '^' => (*x, ((*y - 1) - time).rem_euclid(self.height) + 1),
                    '<' => (((*x - 1) - time).rem_euclid(self.width) + 1, *y),
                    '>' => (((*x - 1) + time).rem_euclid(self.width) + 1, *y),
                    'v' => (*x, ((*y - 1) + time).rem_euclid(self.height) + 1),
                    _ => panic!("Invalid direction for blizzard"),
                };

                if !lut.contains(&new_pos) {
                    lut.insert(new_pos);
                }

                lut
            })
    }

    fn list_options(&self, pos: &Pos, start: Pos, end: Pos, config: &HashSet<Pos>) -> Vec<Pos> {
        let (x, y) = *pos;
        [
            (x, y),
            (x+1, y),
            (x-1, y),
            (x, y+1),
            (x, y-1),
        ]
            .into_iter()
            .filter(|&(tx, ty)| tx > 0 && ty > 0)
            .collect()
    }
}

const DATA_STR: &str = include_str!("../data/day24.example");

pub fn main() {
    println!("[Day24] Solutions:");

    let width = DATA_STR.lines().next().unwrap().len() as i32 - 2;
    let height = DATA_STR.lines().count() as i32 - 2;
    let blizzards = DATA_STR
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().filter_map(move |(x, c)| {
            let pos = (x as i32, y as i32);
            match c {
                '^' | '<' | '>' | 'v' => Some((pos, c)),
                _ => None,
            }
        }))
        .collect();

    let map = Map { width, height, start: (1, 0), end: (width, height + 1), blizzards };

    println!("[Day24] Part 1 => {}", run_part1(&map));
    //println!("[Day24] Part 2 => {}", run_part2());

    println!("[Day24] Complete -----------------------");
}

fn _show_map(map: &Map, time: i32) {
    let blizz_map = map.blizz_for_time(time);

    for y in 0..map.height + 2 {
        for x in 0..map.width + 2 {
            if  blizz_map.contains(&(x, y)) {
                print!(" x ");
            }
            else if x == 0 || x == map.width + 1 || y == 0 || y == map.height + 1 {
                print!(" # ");
            }
            else {
                print!(" . ");
            }
        }
        println!();
    }
}

struct State {
    time: i32,
    pos: Pos,
    visited: HashSet<(i32, Pos)>, // cache of visited states: (config idx; position)
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[at={}; {:?}]\nvisit={:?}", self.time, self.pos, self.visited)
    }
}

fn run_part1(map: &Map) -> i32 {
    println!("Starting from: {:?}, Going to: {:?}", map.start, map.end);

    // Cache all possible configurations
    let mut configs = Vec::<HashSet<Pos>>::new();
    let mut time = 0;
    let mut current = map.blizz_for_time(time);

    while !configs.contains(&current) {
        configs.push(current);

        time += 1;
        current = map.blizz_for_time(time)
    }

    println!("Finished caching configs, we have {} configuration", configs.len());
    println!();
    //for i in 0..configs.len() + 1 {
    //    _show_map(map, i as i32);
    //    println!();
    //}

    // Start exploring paths
    let state = State { time: 0, pos: map.start, visited: HashSet::new() };
    let mut stack = vec![ state ];

    while let Some(s) = stack.pop() {
        println!("Exploring state:\n{}", s);

        println!("Options: ");
        let config = configs.get(s.time as usize % configs.len()).unwrap();
        map.list_options(&s.pos, map.start, map.end, &config).iter().for_each(|o| println!("{:?}", o));
        println!();
    }

    return 0;
}

//fn run_part2() -> usize {
//    return 0;
//}
