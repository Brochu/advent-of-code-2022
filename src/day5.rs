#[derive(Debug)]
struct Move {
    n: u8,
    src: u8,
    dst: u8,
}

pub fn main() {
    println!("[Day5] Solutions:");

    let (layout_data, moves_data) = include_str!("../data/day5.input")
        .split_once("\r\n\r\n")
        .unwrap();

    // Handle initial layout parsing
    let mut init_layout: Vec<Vec<char>> = Vec::new();
    layout_data.lines()
        .for_each(|l| {
            l.chars()
                .enumerate()
                .filter_map(|c| {
                    match c.1 {
                        'A'..='Z' => Some(((c.0 - 1) / 4, c.1)),
                        _ => None,
                    }
                })
                .for_each(|b| {
                    match init_layout.get_mut(b.0) {
                        Some(v) => v.insert(0, b.1),
                        None => {
                            init_layout.resize(b.0 + 1, Vec::<char>::new());
                            init_layout.get_mut(b.0).unwrap().insert(0, b.1);
                        }
                    }
                });
        });

    // Handle moves parsing
    let moves = moves_data.lines()
        .map(|l| {
            let mut it = l.split(' ');
            Move {
                n: it.nth(1).unwrap().parse::<u8>().unwrap(),
                src: it.nth(1).unwrap().parse::<u8>().unwrap(),
                dst: it.nth(1).unwrap().parse::<u8>().unwrap(),
            }
        })
        .collect::<Vec<Move>>();

    println!("[Day5] Part 1 => {:?}", run_part1(&init_layout, &moves));
    println!("[Day5] Part 2 => {:?}", run_part2(&init_layout, &moves));

    println!("[Day5] Complete -----------------------");
}

fn run_part1(init_layout: &Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    let mut layout = init_layout.clone();

    // Apply all the moves
    moves.iter()
        .for_each(|m| {
            for _ in 0..m.n {
                let taken = layout[(m.src-1) as usize].pop().unwrap();
                layout[(m.dst-1) as usize].push(taken);
            }
        });

    return layout.into_iter()
        .map(|mut v| v.pop().unwrap())
        .collect::<String>();
}

fn run_part2(init_layout: &Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    let mut layout = init_layout.clone();

    // Apply all the moves
    moves.iter()
        .for_each(|m| {
            let is = (m.src - 1) as usize;
            let id = (m.dst - 1) as usize;
            let size = layout[is].len() - m.n as usize;

            let taken = layout[is][size..]
                .iter()
                .map(|&c| c)
                .collect::<Vec<char>>();
            layout[is].resize(size, ' ');

            layout[id].extend(taken);
        });

    return layout.into_iter()
        .map(|mut v| v.pop().unwrap())
        .collect::<String>();
}
