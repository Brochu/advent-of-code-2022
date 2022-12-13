use std::collections::VecDeque;
type Map = (Vec<char>, usize);

pub fn main() {
    println!("[Day12] Solutions:");

    let input = include_str!("../data/day12.input");
    let n = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<Vec<char>>();

    let map = (cells, n);

    println!("[Day12] Part 1 => {}", run_part1(&map));
    println!("[Day12] Part 2 => {}", run_part2(&map));

    println!("[Day12] Complete -----------------------");
}

//fn debug_map(map: &Map) {
//    let (cells, n) = map;
//
//    cells[..].chunks(*n)
//        .for_each(|v| {
//            v.iter().for_each(|c| print!(" {c} "));
//            println!();
//        });
//}

fn find_neighbors(map: &Map, idx: usize) -> Vec<(usize, char)> {
    let (cells, n) = map;

    return vec![
        idx as i64 - *n as i64,
        idx as i64 - 1 as i64,
        idx as i64 + 1 as i64,
        idx as i64 + *n as i64,
    ]
    .into_iter()
    .filter(|&i| {
        if i >= 0 && (i as usize) < cells.len() {
            let curr_height = match cells[idx] {
                'S' => b'a' as u8,
                'E' => b'z' as u8,
                _ => cells[idx] as u8,
            };
            let target_height = match cells[i as usize] {
                'S' => b'a' as u8,
                'E' => b'z' as u8,
                _ => cells[i as usize] as u8,
            };

            return target_height <= curr_height + 1;
        }
        else {
            false
        }
    })
    .map(|i| (i as usize, cells[i as usize]))
    .collect::<Vec<(usize, char)>>();
}

fn shortest_path(map: &Map, start: usize, end: usize) -> usize {
    let (cells, _) = map;

    // Prepare the distance map
    let mut dist = cells.iter()
        .enumerate()
        .map(|(idx, _)| { if idx == start { 0 } else { usize::MAX } })
        .collect::<Vec<usize>>();
    let mut parents = cells.iter()
        .enumerate()
        .map(|(_, _)| None)
        .collect::<Vec<Option<usize>>>();

    // Prepare the stack of idx to visit
    let mut stack = VecDeque::new();
    stack.push_back(start);

    while let Some(current) = stack.pop_front() {
        let cost = dist[current] + 1;

        for n in find_neighbors(map, current) {
            let (n_idx, _) = n;

            if cost < dist[n_idx] {
                // We just found a better way
                dist[n_idx] = cost;
                parents[n_idx] = Some(current);
                stack.push_back(n_idx);
            }
        }
    }

    //dist.iter().zip(parents.iter())
    //    .enumerate()
    //    .for_each(|(k, (d, p))| println!("{k} -> {d}; {:?}", p));

    return dist[end];
}

fn run_part1(map: &Map) -> usize {
    let (cells, _) = map;
    let (start, end) = cells.iter()
        .enumerate()
        .filter(|(_, c)| c.is_uppercase())
        .fold((0 as usize, 0 as usize), |tup, (idx, c)| {
            match c {
                'S' => (idx, tup.1),
                'E' => (tup.0, idx),
                _ => panic!("Does not match either 'S' or 'E' ..."),
            }
        });

    return shortest_path(map, start, end);
}

fn run_part2(map: &Map) -> usize {
    let (cells, _) = map;
    let end = cells.iter().enumerate().find(|(_, &c)| c == 'E').unwrap().0;

    return cells.iter()
        .enumerate()
        .filter_map(|(start, &c)| {
            if c == 'a' || c == 'S' {
                Some(shortest_path(map, start, end))
            }
            else {
                None
            }
        })
        .min().unwrap();
}
