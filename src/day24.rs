use std::collections::{ HashMap, HashSet, BinaryHeap };

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
    fn _blizz_for_time(&self, time: i32) -> HashMap<Pos, Vec<char>> {
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

    fn list_blizzards(&self, time: i32, pos: Pos) -> HashSet<Pos> {
        //TODO: Cache all possible config of blizzards since there will be a cycle at one point
        self.blizzards.iter()
            .filter(|&((x, y), dir)| {
                *x >= (pos.0 - 1) && *x <= (pos.0 + 1) && (*dir == '^' || *dir == 'v') ||
                *y >= (pos.1 - 1) && *y <= (pos.1 + 1) && (*dir == '<' || *dir == '>')
            })
            .fold(HashSet::new(), |mut lut, ((x, y), dir)| {
                let new_pos = match dir {
                    '^' => (*x, ((*y - 1) - time).rem_euclid(self.height) + 1),
                    '<' => (((*x - 1) - time).rem_euclid(self.width) + 1, *y),
                    '>' => (((*x - 1) + time).rem_euclid(self.width) + 1, *y),
                    'v' => (*x, ((*y - 1) + time).rem_euclid(self.height) + 1),
                    _ => panic!("Invalid direction for blizzard"),
                };

                lut.insert(new_pos);
                lut
            })
    }

    fn list_options(&self, state: &State) -> Vec<Pos> {
        let (x, y) = state.pos;
        let blizzards = self.list_blizzards(state.time + 1, state.pos);

        return vec![ (x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1), (x, y) ]
            .iter()
            .filter_map(|&pos| { 
                if !blizzards.contains(&pos) &&
                    !state.visited.contains(&pos) &&
                    pos.0 > 0 && pos.1 > 0 &&
                    pos.0 <= self.width && pos.1 <= self.height || 
                    (pos.0 == self.end.0 && pos.1 == self.end.1) ||
                    (pos.0 == self.start.0 && pos.1 == self.start.1)
                {
                    Some(pos)
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
    priority: i32,
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(other.priority.cmp(&self.priority));
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.priority.cmp(&self.priority);
    }
}

impl std::cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        return self.priority == other.priority;
    }
}

impl std::cmp::Eq for State {
}

const DATA_STR: &str = include_str!("../data/day24.input");

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
    let blizz_map = map._blizz_for_time(time);

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

fn dist(p0: Pos, p1: Pos) -> i32 {
    let dx = (p1.0 - p0.0).abs();
    let dy = (p1.1 - p0.1).abs();
    return dx + dy;
}

fn run_part1(map: &Map) -> i32 {
    //_show_map(map, 0);

    println!("Starting from: {:?}, Going to: {:?}", map.start, map.end);
    println!();

    let mut stack = BinaryHeap::from(vec![
        State { pos: map.start,
            time: 0,
            visited: HashSet::new(), //TODO: We need to keep track of blizzard config as well as pos
            priority: dist(map.start, map.end),
        }
    ]);
    let mut min_time: i32 = i32::MAX;

    while let Some(s) = stack.pop() {
        println!("[time = {}][at = {:?}][priority = {}]", s.time, s.pos, s.priority);

        let opts = map.list_options(&s);
        if opts.iter().any(|&pos| pos == map.end){
            if s.time < min_time { min_time = s.time + 1 }
            continue;
        }

        if s.time > min_time {
            continue;
        }

        opts.iter()
            .for_each(|&pos| {
                // Update state with next position
                    let mut state = s.clone();
                    state.pos = pos;
                    state.time += 1;
                    state.priority = dist(pos, map.end);

                    stack.push(state);
            });
    }

    return min_time;
}

//fn run_part2() -> usize {
//    return 0;
//}
