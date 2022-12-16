//#[derive(Debug)]
struct Sensor {
    pos: (i64, i64),
    //beacon: (i64, i64),
    range: u64,
}

type Map = Vec<Sensor>;

//fn draw_map(map: &Map) {
//    let (mut x_min, mut x_max) = (i64::MAX, 0);
//    let (mut y_min, mut y_max) = (i64::MAX, 0);
//
//    map.iter()
//        .for_each(|s| {
//            let (pos_x, pos_y) = s.pos;
//            let (b_x, b_y) = s.beacon;
//
//            let x_mins = vec![x_min, pos_x, b_x];
//            let y_mins = vec![y_min, pos_y, b_y];
//
//            x_min = *x_mins.iter().min().unwrap();
//            y_min = *y_mins.iter().min().unwrap();
//
//            let x_maxs = vec![x_max, pos_x, b_x];
//            let y_maxs = vec![y_max, pos_y, b_y];
//
//            x_max = *x_maxs.iter().max().unwrap();
//            y_max = *y_maxs.iter().max().unwrap();
//        });
//
//    for y in y_min..=y_max {
//        for x in x_min..=x_max {
//            let c = (x, y);
//            let found = map.iter().filter(|s| s.pos == c || s.beacon == c).next();
//
//            let out = match found {
//                Some(s) => {
//                    if s.pos == c { "S" } else { "B" }
//                },
//                None => ".",
//            };
//
//            print!("{out}");
//        }
//        println!();
//    }
//}

pub fn main() {
    println!("[Day15] Solutions:");
    let (input, row) = (include_str!("../data/day15.example"), 10);
    //let (input, row) = (include_str!("../data/day15.input"), 2_000_000);

    let map = input.lines()
        .map(|line| {
            // Is there a way to make this less ugly??
            let (sensor_str, beacon_str) = line.split_once(": ").unwrap();
            let (_, pos_str) = sensor_str.split_once("at ").unwrap();
            let (_, closest_str) = beacon_str.split_once("at ").unwrap();

            let (x_str, y_str) = pos_str.split_once(", ").unwrap();
            let (_, x) = x_str.split_once("=").unwrap();
            let (_, y) = y_str.split_once("=").unwrap();
            let pos = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());

            let (x_str, y_str) = closest_str.split_once(", ").unwrap();
            let (_, x) = x_str.split_once("=").unwrap();
            let (_, y) = y_str.split_once("=").unwrap();
            let beacon = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());

            Sensor { pos, /*beacon,*/ range: ((pos.0 - beacon.0).abs() + (pos.1 - beacon.1).abs()) as u64 }
        })
    .collect::<Map>();

    //map.iter()
    //    .for_each(|s| println!("pos: {:?}, beacon: {:?}, range: {}", s.pos, s.beacon, s.range));

    println!("[Day15] Part 1 => {}", run_part1(&map, row));
    println!("[Day15] Part 2 => {}", run_part2(&map));

    println!("[Day15] Complete -----------------------");
}

fn cover_ranges(map: &Map, row: i64) -> Vec<(i64, i64)> {
    //draw_map(map);
    let mut ranges =  map.iter()
        .filter_map(|s| {
            let (x, y) = s.pos;
            let dist = (y - row).abs() as u64;

            if  dist <= s.range {
                let diff = (s.range - dist) as i64;
                Some((x - diff, x + diff))
            }
            else {
                None
            }
        })
    .collect::<Vec<(i64, i64)>>();

    ranges.sort_by(|&(l_start, _), &(r_start, _)| l_start.cmp(&r_start));
    return ranges;
}

fn run_part1(map: &Map, row: i64) -> usize {
    let (covered_min, covered_max) = cover_ranges(map, row)
        .iter()
        .fold((i64::MAX, i64::MIN), |(final_min, final_max), &(current_min, current_max)| {
            (
                if current_min < final_min { current_min } else { final_min },
                if current_max > final_max { current_max } else { final_max },
            )
        });

    return (covered_min..covered_max).count();
}

fn _find_beacon(map: &Map) -> (i64, i64) {
    //const MAX: i64 = 20;
    const MAX: i64 = 4_000_000;

    // THIS DOES NOT WORK, takes too long, work with merged ranges instead
    for row in 0..=MAX {
        let ranges = cover_ranges(map, row);

        for x in 0..=MAX {

            if !ranges.iter().map(|&(min, max)| min..=max).any(|r| r.contains(&(x as i64))) {
                return (x, row);
            }
        }
    }

    return (-1, -1);
}

fn merge_covers(map: &Map, row: i64) -> Vec<(i64, i64)> {
    let covers = cover_ranges(map, row);
    let mut ranges = Vec::<(i64, i64)>::new();

    for i in 0..covers.len() {
        let mut found = false;

        for j in 0..ranges.len() {
            let (cover_min, cover_max) = covers[i];
            let (range_min, range_max) = ranges[j];

            if (cover_min >= range_min && cover_min <= range_max) ||
                (cover_max >= range_min && cover_max <= range_max) {

                ranges.swap_remove(j);
                ranges.push((cover_min.min(range_min), cover_max.max(range_max)));
                found = true;
            }
        }

        if !found {
            ranges.push(covers[i].clone());
        }
    }

    return ranges;
}

fn find_beacon(map: &Map) -> (i64, i64) {
    for i in 0..=4_000_000 {
        let test = merge_covers(map, i);
        if test.len() > 1 {
            return (test[0].1 + 1, i);
        }
    }

    return (-1, -1);
}

fn run_part2(map: &Map) -> i64 {
    let (beacon_x, beacon_y) = find_beacon(map);
    return (beacon_x * 4_000_000) + beacon_y;
}
