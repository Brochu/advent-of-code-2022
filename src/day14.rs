use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Cell {
    Stone,
    //Sand,
}

type Segment = Vec<(u32, u32)>;
type Map = HashMap<(u32, u32), Cell>;

fn build_map(segments: &Vec<Segment>) -> Map {
    let mut min = (u32::MAX, u32::MAX);
    let mut max = (u32::MIN, u32::MIN);

    let map = segments.iter()
        .fold(Map::new(), |mut m, seg| {
            m.insert(seg[0], Cell::Stone);

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
                        m.insert((x, y), Cell::Stone);
                    }
                }
            });
            m
        });

    // Find map bounds here?
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
    //println!("[Day14] Part 2 => {}", run_part2(&lines));

    println!("[Day14] Complete -----------------------");
}

fn run_part1(map: Map) -> usize {
    map.iter()
        .for_each(|(k, v)| {
            println!("{:?} -> {:?}", k, v);
        });

    return map.len();
}
//
//fn run_part2(lines: &Vec<&str>) -> usize {
//    return lines.len();
//}
