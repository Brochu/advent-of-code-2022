use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Cell {
    Spawner,
    Stone,
    Sand,
}

type Segment = Vec<(u32, u32)>;

#[derive(Clone)]
struct Map {
    cells: HashMap<(u32, u32), Cell>,
    min_bounds: (u32, u32),
    max_bounds: (u32, u32),
    spawn: (u32, u32),
}

impl Map {
    pub fn show_map(&self, part2: bool) {
        let (x_min, y_min) = self.min_bounds;
        let (x_max, y_max) = self.max_bounds;

        for y in y_min..=y_max+3 {
            for x in x_min-2..=x_max+2 {
                let cell = match self.get_cell(&(x, y), part2) {
                    Some(c) => {
                        match c {
                            Cell::Spawner => '+',
                            Cell::Stone => '#',
                            Cell::Sand => 'o',
                        }
                    },
                    None => '.', // Part 2
                };
                print!("{}", cell);
            }
            println!();
        }
    }

    pub fn get_cell(&self, coord: &(u32, u32), part2: bool) -> Option<&Cell> {
        let &(_, y) = coord;
        let (_, y_max) = self.max_bounds;

        if y == y_max + 2 && part2 {
            return Some(&Cell::Stone);
        }
        else {
            return self.cells.get(coord);
        }
    }
}

fn build_map(segments: &Vec<Segment>) -> Map {
    let mut map = segments.iter()
        .fold(Map { cells: HashMap::new(),
                    min_bounds: (u32::MAX, 0),
                    max_bounds: (0, 0),
                    spawn: (500, 0) }, |mut m, seg| {
            m.cells.insert(seg[0], Cell::Stone);

            seg[..].windows(2).for_each(|w| {
                let (x0, y0) = w[0];
                let (x1, y1) = w[1];

                let xrange = match x0 < x1 {
                    true => x0..=x1,
                    false => x1..=x0,
                };
                let yrange = match y0 < y1 {
                    true => y0..=y1,
                    false => y1..=y0,
                };

                for x in xrange {
                    for y in yrange.clone() {
                        m.cells.insert((x, y), Cell::Stone);

                        if x < m.min_bounds.0 { m.min_bounds.0 = x; }
                        if x > m.max_bounds.0 { m.max_bounds.0 = x; }

                        if y < m.min_bounds.1 { m.min_bounds.1 = y; }
                        if y > m.max_bounds.1 { m.max_bounds.1 = y; }
                    }
                }
            });
            m
        });

    map.cells.insert(map.spawn, Cell::Spawner);
    return map;
}

fn find_next_move(map: &Map, current: (u32, u32), part2: bool) -> Option<(u32, u32)> {
    let (x, y) = current;
    let possible_targets = vec![(x, y+1), (x-1, y+1), (x+1, y+1)];

    for p in possible_targets {
        if let None = map.get_cell(&p, part2) {
            return Some(p);
        }
    }

    //TODO: Make sure to handle infinite plane at max_bounds.y + 2 later for part 2

    return None;
}

fn simul_sand(map: &Map, start: (u32, u32), part2: bool) -> (bool, (u32, u32)) {
    let (mut x, mut y) = start;

    while y < map.max_bounds.1 + 3 {
        //println!("Sand at ({:?}), falling to ({:?})", (x, y), target);
        match find_next_move(&map, (x, y), part2) {
            Some((new_x, new_y)) => {
                x = new_x;
                y = new_y;
            },
            None => {
                if (x, y) == start {
                    return (true, start);
                }
                else {
                    return (false, (x, y));
                }
            }
        };
    }

    return (true, (0, 0));
}

pub fn main() {
    println!("[Day14] Solutions:");

    let segments = include_str!("../data/day14.input")
        .lines().map(|line| {
            line.split(" -> ").map(|e| {
                    let (x, y) = e.split_once(",").unwrap();
                    (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
                })
                .collect::<Segment>()
        })
        .collect::<Vec<Segment>>();

    let map = build_map(&segments);
    println!("[Day14] Part 1 => {}", run_part1(map.clone()));
    println!("[Day14] Part 2 => {}", run_part2(map.clone()));

    println!("[Day14] Complete -----------------------");
}

fn run_part1(mut map: Map) -> usize {
    for _ in 0..100_000 {
        let (finished, target) = simul_sand(&map, map.spawn, false);

        if !finished {
            map.cells.insert(target, Cell::Sand);
        }
        else {
            break;
        }
    }

    map.show_map(false);
    return map.cells.iter()
        .filter(|&(_, cell)| {
            match cell {
                Cell::Sand => true,
                _ => false,
            }
        })
        .count();
}

fn run_part2(mut map: Map) -> usize {
    for _ in 0..100_000 {
        let (finished, target) = simul_sand(&map, map.spawn, true);

        if !finished {
            map.cells.insert(target, Cell::Sand);
        }
        else {
            break;
        }
    }

    map.show_map(true);
    return map.cells.iter()
        .filter(|&(_, cell)| {
            match cell {
                Cell::Sand => true,
                _ => false,
            }
        })
        .count() + 1;
}
