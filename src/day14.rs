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

//impl Map {
//    pub fn show_map(&self) {
//        let (x_min, y_min) = self.min_bounds;
//        let (x_max, y_max) = self.max_bounds;
//
//        for y in y_min..=y_max+2 {
//            for x in x_min-2..=x_max+2 {
//                let cell = match self.cells.get(&(x, y)) {
//                    Some(c) => {
//                        match c {
//                            Cell::Spawner => '+',
//                            Cell::Stone => '#',
//                            Cell::Sand => 'o',
//                        }
//                    },
//                    None => '.',
//                };
//                print!("{}", cell);
//            }
//            println!();
//        }
//    }
//}

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

pub fn main() {
    println!("[Day14] Solutions:");

    let segments = include_str!("../data/day14.example")
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
    //TODO: Find a way to split this, reuse for part 2
    for _ in 0..100_000 {
        let (mut x, mut y) = map.spawn;
        let mut fell = true;

        while y < map.max_bounds.1 {
            let target = match map.cells.get(&(x, y+1)) {
                None => Some((x, y+1)),
                Some(_) => {
                    match map.cells.get(&(x-1, y+1)) {
                        None => Some((x-1, y+1)),
                        Some(_) => {
                            match map.cells.get(&(x+1, y+1)) {
                                None => Some((x+1, y+1)),
                                Some(_) => None,
                            }
                        },
                    }
                },
            };

            //println!("Sand at ({:?}), falling to ({:?})", (x, y), target);
            match target {
                Some((new_x, new_y)) => {
                    x = new_x;
                    y = new_y;
                },
                None => {
                    map.cells.insert((x, y), Cell::Sand);
                    fell = false;
                    break;
                }
            };
        }

        if fell {
            break;
        }
    }

    //map.show_map();
    return map.cells.iter()
        .filter(|&(_, cell)| {
            match cell {
                Cell::Sand => true,
                _ => false,
            }
        })
        .count();
}

fn run_part2(mut _map: Map) -> usize {
    // Simulate infinite floor at max_bounds.1 + 2
    return 0;
}
