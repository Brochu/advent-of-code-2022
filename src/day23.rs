use std::collections::{ HashSet, HashMap };

type Elf = (i32, i32);
type Lut = HashSet<(i32, i32)>;

#[derive(Debug)]
enum Position {
    N, NW, NE,
    S, SW, SE,
    W, E,
}

pub fn main() {
    println!("[Day23] Solutions:");

    let elves = include_str!("../data/day23.example").lines().enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate()
                .filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((x as i32, y as i32))
                    }
                    else {
                        None
                    }
                })
        })
        .collect::<Vec<Elf>>();

    println!("[Day23] Part 1 => {}", run_part1(&elves));
    println!("[Day23] Part 2 => {}", run_part2(&elves));

    println!("[Day23] Complete -----------------------");
}

fn expand_positions(elf: &Elf, pos: &Vec<Position>) -> Vec<(i32, i32)> {
    let &(x, y) = elf;
    return pos.iter()
        .map(|p| {
            match p {
                Position::N => (x, y-1),
                Position::NW => (x-1, y-1),
                Position::NE => (x+1, y-1),

                Position::S => (x, y+1),
                Position::SW => (x-1, y+1),
                Position::SE => (x+1, y+1),

                Position::W => (x-1, y),
                Position::E => (x+1, y),
            }
        })
        .collect();
}

fn is_elf_done(elf: &Elf, lut: &Lut) -> bool {
    let pos = vec![
        Position::N, Position::NW, Position::NE,
        Position::S, Position::SW, Position::SE,
        Position::W, Position::E,
    ];

    return !expand_positions(elf, &pos).iter().any(|p| lut.contains(p));
}

fn potential_move(elf: &Elf, lut: &Lut, rules: &Vec<Vec<Position>>, r: usize) -> (i32, i32) {
    for i in 0..rules.len() {
        let rule = &rules[(i + r) % rules.len()];

        let positions = expand_positions(elf, rule);
        if positions.iter().all(|p| !lut.contains(p)) {
            return *positions.first().unwrap();
        }
    }

    return (0, 0);
}

fn get_bounds(elves: &Vec<Elf>) -> ((i32, i32), (i32, i32)) {
    let (min_x, max_x) = elves.iter().fold((i32::MAX, i32::MIN), |(min, max), &(x, _)| {
        if x < min { (x, max) }
        else if x > max { (min, x) }
        else { (min, max) }
    });
    let (min_y, max_y) = elves.iter().fold((i32::MAX, i32::MIN), |(min, max), &(_, y)| {
        if y < min { (y, max) }
        else if y > max { (min, y) }
        else { (min, max) }
    });

    return ((min_x, min_y), (max_x, max_y));
}

fn run_part1(elves: &Vec<Elf>) -> i32 {
    let mut elves = elves.clone();

    let rules = vec![
        vec![Position::N, Position::NE, Position::NW],
        vec![Position::S, Position::SE, Position::SW],
        vec![Position::W, Position::NW, Position::SW],
        vec![Position::E, Position::NE, Position::SE],
    ];

    const ROUNDS: usize = 10;
    for r in 0..ROUNDS {
        let mut moves: HashMap<(i32, i32), Vec<Elf>> = HashMap::new();
        // Maybe there is a better way to handle this?
        let lut = elves.iter().map(|&p| p).collect::<HashSet<Elf>>();

        for elf in &elves[..] {
            if is_elf_done(elf, &lut) { continue }
            let m = potential_move(elf, &lut, &rules, r);

            if let Some(list) = moves.get_mut(&m) {
                list.push(*elf);
            }
            else {
                moves.insert(m, vec![*elf]);
            }
        }

        moves.iter()
            .filter_map(|(target, list)| {
                if list.len() == 1 { Some((target, list.first().unwrap())) }
                else { None }
            })
            .for_each(|(target, &source)| {
                let e = elves.iter_mut().find(|e| **e == source).unwrap();
                *e = *target;
            });
    }

    let (min, max) = get_bounds(&elves);
    let x_len = (max.0 + 1) - min.0;
    let y_len = (max.1 + 1) - min.1;

    for y in -3..y_len {
        for x in -3..x_len {
            if elves.contains(&(x, y)) {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    return (x_len * y_len) - elves.len() as i32;
}

fn run_part2(elves: &Vec<Elf>) -> i32 {
    let mut elves = elves.clone();

    let rules = vec![
        vec![Position::N, Position::NE, Position::NW],
        vec![Position::S, Position::SE, Position::SW],
        vec![Position::W, Position::NW, Position::SW],
        vec![Position::E, Position::NE, Position::SE],
    ];

    let mut r = 0;
    let round = loop {
        let mut moves: HashMap<(i32, i32), Vec<Elf>> = HashMap::new();
        // Maybe there is a better way to handle this?
        let lut = elves.iter().map(|&p| p).collect::<HashSet<Elf>>();

        for elf in &elves[..] {
            if is_elf_done(elf, &lut) { continue }
            let m = potential_move(elf, &lut, &rules, r);

            if let Some(list) = moves.get_mut(&m) {
                list.push(*elf);
            }
            else {
                moves.insert(m, vec![*elf]);
            }
        }

        let mut moved = false;
        moves.iter()
            .filter_map(|(target, list)| {
                if list.len() == 1 { Some((target, list.first().unwrap())) }
                else { None }
            })
            .for_each(|(target, &source)| {
                let e = elves.iter_mut().find(|e| **e == source).unwrap();
                *e = *target;
                moved = true;
            });

        if !moved {
            break r;
        }
        r+=1;
    };

    //TODO: Look into this, gives too high answer
    //  When do the elves stop moving, why are we still reporting moves
    return (round+1) as i32;
}
