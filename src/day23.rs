type Elf = (i32, i32);

#[derive(Debug)]
enum Position {
    N, NW, NE,
    S, SW, SE,
    W, E,
}

pub fn main() {
    println!("[Day23] Solutions:");

    let elves = include_str!("../data/day23.example.small").lines().enumerate()
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
    //println!("[Day23] Part 2 => {}", run_part2());

    println!("[Day23] Complete -----------------------");
}

fn is_elf_done(_elf: &Elf) -> bool {
    //TODO: Check all 8 dirs around
    return false;
}

fn potential_move(_elf: &Elf, rules: &Vec<Vec<Position>>, r: usize) -> (i32, i32) {
    //TODO: Check rules in order based on round
    //  If no elves are in the directions checked, select move based on dirs checked
    //  Return directly when finding one valid potential move

    for i in 0..rules.len() {
        let rule = &rules[(i + r) % rules.len()];
        print!("[{r}] Looking at rule :");
        rule.iter().for_each(|c| print!("{:?}, ", c));
        println!();
    }
    println!();

    return (0, 0);
}

fn run_part1(elves: &Vec<Elf>) -> i64 {
    let rules = vec![
        vec![Position::N, Position::NE, Position::NW],
        vec![Position::S, Position::SE, Position::SW],
        vec![Position::W, Position::NW, Position::SW],
        vec![Position::E, Position::NE, Position::SE],
    ];

    const ROUNDS: usize = 1;
    for r in 0..ROUNDS {
        println!("[{r}] Round first half");
        let mut moves = vec![];

        for elf in elves {
            if is_elf_done(elf) { continue }
            let m = potential_move(elf, &rules, r);

            //TODO: Check if this is a unique move for the round
            moves.push((elf, m));
        }

        println!("[{r}] Round second half");
        //TODO: Apply valid moves here
        moves.iter()
            .for_each(|m| println!("{:?}", m));
    }

    return elves.len() as i64;
}

//fn run_part2() -> usize {
//    return 0;
//}
