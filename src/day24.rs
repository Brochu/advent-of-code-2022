use std::collections::{ HashMap, HashSet };

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
    fn blizz_for_time(&self, time: i32) -> HashMap<Pos, Vec<char>> {
        self.blizzards.iter()
            .fold(HashMap::new(), |mut lut, ((x, y), dir)| {
                let new_pos = match dir {
                    '^' => (*x, ((*y - 1) - time).rem_euclid(self.height) + 1),
                    '<' => (((*x - 1) - time).rem_euclid(self.width) + 1, *y),
                    '>' => (((*x - 1) + time).rem_euclid(self.width) + 1, *y),
                    'v' => (*x, ((*y - 1) + time).rem_euclid(self.height) + 1),
                    _ => panic!("Invalid direction for blizzard"),
                };


                if let Some(list) = lut.get_mut(&new_pos) {
                    list.push(*dir);
                }
                else {
                    lut.insert(new_pos, vec![*dir]);
                }

                lut
            })
    }

    fn list_options(&self, pos: Pos, visited: &HashSet<Pos>) -> Vec<Pos> {
        let (x, y) = pos;

        return vec![ (x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1), (x, y) ]
            .iter()
            .filter_map(|(tx, ty)| { 
                if !visited.contains(&(*tx, *ty)) &&
                    *tx > 0 && *ty > 0 &&
                    *tx <= self.width && *ty <= self.height || 
                    (*tx == self.end.0 && *ty == self.end.1)
                {

                    Some((*tx, *ty))
                }
                else {
                    None
                }
            })
            .collect();
    }
}

#[derive(Clone)]
struct State {
    pos: Pos,
    time: i32,
    visited: HashSet<Pos>,
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

fn show_map(map: &Map, time: i32) {
    let blizz_map = map.blizz_for_time(time);

    for y in 0..map.height + 2 {
        for x in 0..map.width + 2 {
            if let Some(list) = blizz_map.get(&(x, y)) {
                if list.len() > 1 {
                    print!(" {} ", list.len());
                }
                else {
                    print!(" {} ", list[0]);
                }
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


fn run_part1(map: &Map) -> i32 {
    show_map(map, 0);

    println!("Starting from: {:?}, Going to: {:?}", map.start, map.end);
    println!();

    let mut stack = vec![
        State{ pos: map.start, time: 0, visited: HashSet::from_iter(vec![ map.start ].into_iter()) }
    ];
    let mut min_time: i32 = i32::MAX;

    while let Some(s) = stack.pop() {
        let opts = map.list_options(s.pos, &s.visited);

        if opts.iter().any(|&pos| pos == map.end){
            println!("[time = {}][at = {:?}]: {:?}", s.time, s.pos, s.visited);
            if s.time < min_time { min_time = s.time + 1 }
            continue;
        }

        opts.iter()
            .for_each(|pos| {
                // Update state with next position
                let mut state = s.clone();
                state.visited.insert(*pos);
                state.pos = *pos;
                state.time += 1;

                stack.push(state);
            });
    }

    return min_time;
}

//fn run_part2() -> usize {
//    return 0;
//}
