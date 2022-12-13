type Map = (Vec<char>, usize);

pub fn main() {
    println!("[Day12] Solutions:");

    let input = include_str!("../data/day12.example");
    let n = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<Vec<char>>();

    let map = (cells, n);

    println!("[Day12] Part 1 => {}", run_part1(&map));
    println!("[Day12] Part 2 => {}", run_part2());

    println!("[Day12] Complete -----------------------");
}

fn debug_map(map: &Map) {
    let (cells, n) = map;

    cells[..].chunks(*n)
        .for_each(|v| {
            v.iter().for_each(|c| print!(" {c} "));
            println!();
        });
}

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
        if i >= 0 {
            let curr_height = match cells[idx] {
                'S' => b'a' as u8,
                'E' => b'z' as u8,
                _ => cells[idx] as u8,
            };
            let target_height = cells[i as usize] as u8;

            return target_height <= curr_height + 1;
        }
        else {
            false
        }
    })
    .map(|i| (i as usize, cells[i as usize]))
    .collect::<Vec<(usize, char)>>();
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

    debug_map(map);
    println!("Start from: {}; End at: {}", start, end);

    let n = find_neighbors(map, 27);
    println!("Neighbors : {:?}", n);
    return cells.len();
}

fn run_part2() -> usize {
    return 0;
}
