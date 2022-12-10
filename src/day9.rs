use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

#[derive(Debug)]
struct Pos {
    x: i64,
    y: i64,
}

impl ToString for Pos {
    fn to_string(&self) -> String {
        return format!("({}, {})", self.x, self.y);
    }
}

//fn show_grid(head: &Pos, tail: &Pos) {
//    println!("Head: {}; Tail: {}", head.to_string(), tail.to_string());
//}

fn show_grid_part2(positions: &[Rc<RefCell<Pos>>], dims: usize) {
    let mut output = Vec::<char>::new();
    output.resize(dims * dims, '.');

    positions.iter()
        .enumerate()
        .rev()
        .for_each(|(i, p)| {
            let pos = p.borrow();
            let idx = ((dims * dims) - 1) - ((pos.y * dims as i64) + pos.x) as usize;

            let letter = match i {
                0 => 'H',
                9 => 'T',
                _ => i.to_string()[0..1].chars().nth(0).unwrap(),
            };
            output[idx] = letter;
        });

    output[..].chunks(dims)
        .for_each(|c| {
            c.iter().rev()
                .for_each(|v| print!("{}", v));
            println!();
        });
}

fn should_move_tail(head: &Pos, tail: &Pos) -> bool {
    // Same column or row cases
    if head.x == tail.x {
        return (tail.y - head.y).abs() >= 2;
    }
    else if head.y == tail.y {
        return (tail.x - head.x).abs() >= 2;
    }
    else {
        // Diagonal move possible
        return ((tail.x - head.x).abs() + (tail.y - head.y).abs()) > 2;
    }
}

fn new_tail_pos(head: &Pos, tail: &Pos, _m: &Move) -> Pos {
    let (xdiff, ydiff) = (head.x - tail.x, head.y - tail.y);

    if xdiff == 0 || ydiff == 0 {
        return Pos { x: tail.x + (xdiff/2), y: tail.y + (ydiff/2) };
    }
    else {
        //println!("Diag case ({}, {}), ({}, {}) -> ({}, {}) [{:?}]", head.x, head.y, tail.x, tail.y, xdiff, ydiff, m);
        //return match m {
        //    Move::Up(_) => Pos { x: tail.x + xdiff, y: tail.y + 1 },
        //    Move::Down(_) => Pos { x: tail.x + xdiff, y: tail.y - 1 },
        //    Move::Left(_) => Pos { x: tail.x - 1, y: tail.y + ydiff },
        //    Move::Right(_) => Pos { x: tail.x + 1, y: tail.y + ydiff },
        //};
        let xmod = xdiff.signum();
        let ymod = ydiff.signum();

        return Pos { x: tail.x + xmod, y: tail.y + ymod };
    }
}

pub fn main() {
    println!("[Day9] Solutions:");

    let moves = include_str!("../data/day9.input")
        .lines()
        .map(|line| {
            let (dir, nstr) = line.split_once(" ").unwrap();
            let n = nstr.parse::<usize>().unwrap();

            match dir {
                "U" => Move::Up(n),
                "D" => Move::Down(n),
                "L" => Move::Left(n),
                "R" => Move::Right(n),
                _ => panic!("[Day9] Count not parse move with this direction"),
            }
        })
        .collect::<Vec<Move>>();

    println!("[Day9] Part 1 => {}", run_part1(&moves));
    println!("[Day9] Part 2 => {}", run_part2(&moves));

    println!("[Day9] Complete -----------------------");
}

fn run_part1(moves: &Vec<Move>) -> usize {
    let mut head = Pos{ x: 0, y: 0 };
    let mut tail = Pos{ x: 0, y: 0 };
    let mut visited = HashSet::<(i64, i64)>::new();

    moves.iter()
        .for_each(|m| {
            let (xmod, ymod, range) = match *m {
                Move::Up(n) => (0, 1, 0..n),
                Move::Down(n) => (0, -1, 0..n),
                Move::Left(n) => (-1, 0, 0..n),
                Move::Right(n) => (1, 0, 0..n),
            };

            for _ in range {
                head.x += xmod;
                head.y += ymod;

                if should_move_tail(&head, &tail) {
                    tail = new_tail_pos(&head, &tail, m);
                    visited.insert((tail.x, tail.y));
                }
            }
        });

    return visited.len() + 1;
}

fn run_part2(moves: &Vec<Move>) -> usize {
    let count = 10;

    let mut pos: Vec<Rc<RefCell<Pos>>> = Vec::new();
    for _ in 0..count {
        pos.push(Rc::new(RefCell::new(Pos { x: 0, y: 0 })));
    }

    let mut visited = HashSet::<(i64, i64)>::new();

    moves.iter()
        .for_each(|m| {
            let (xmod, ymod, range) = match *m {
                Move::Up(n) => (0, 1, 0..n),
                Move::Down(n) => (0, -1, 0..n),
                Move::Left(n) => (-1, 0, 0..n),
                Move::Right(n) => (1, 0, 0..n),
            };

            for _ in range {
                // Update head position
                pos[0].borrow_mut().x += xmod;
                pos[0].borrow_mut().y += ymod;

                (0..pos.len()).collect::<Vec<usize>>()
                    .windows(2)
                    .for_each(|w| {
                        let last = Rc::clone(&pos[w[0]]);
                        let current = Rc::clone(&pos[w[1]]);

                        if should_move_tail(&last.borrow(), &current.borrow()) {
                            let new_pos = new_tail_pos(&last.borrow(), &current.borrow(), m);
                            let mut cur_mut = current.borrow_mut();
                            cur_mut.x = new_pos.x;
                            cur_mut.y = new_pos.y;

                            if w[1] == 9 {
                                visited.insert((cur_mut.x, cur_mut.y));
                            }
                        }
                    });

                //show_grid_part2(&pos[..], 45);
                //println!();
            }
        });

    return visited.len() + 1;
}
