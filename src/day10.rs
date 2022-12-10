#[derive(Debug)]
enum Op {
    Exit,
    Noop,
    Addx(i64),
}

pub fn main() {
    println!("[Day10] Solutions:");

    let program = include_str!("../data/day10.input")
        .lines()
        .map(|line| {
            let inst = &line[0..4];

            match inst {
                "noop" => Op::Noop,
                "addx" => Op::Addx(line[5..].trim().parse::<i64>().unwrap()),
                _ => panic!("[Day10] Could not parse instruction"),
            }
        })
        .collect::<Vec<Op>>();

    println!("[Day10] Part 1 => {}", run_part1(&program));
    //println!("[Day10] Part 2 => {}", run_part2(&lines));

    println!("[Day10] Complete -----------------------");
}

fn num_cycles(op: &Op) -> usize {
    match op {
        Op::Exit => 0,
        Op::Noop => 1,
        Op::Addx(_) => 2,
    }
}

fn run_part1(program: &Vec<Op>) -> i64 {
    let mut prog_iter = program.iter();
    let mut pc: Option<(&Op, usize)> = None;
    let mut x: i64 = 1;
    
    let mut samples: Vec<i64> = Vec::new();

    while let mut current = pc.unwrap_or_else(|| {
        match prog_iter.next() {
            Some(op) => (op, num_cycles(op)),
            None => (&Op::Exit, 0),
        }
    })

    {
        samples.push(x);
        //println!("[{}][{}] {:?} ({} clocks)", cl, x, current.0, current.1);

        match current.0 {
            Op::Exit => break,
            Op::Noop => { pc = None; },
            Op::Addx(val) => {
                current.1 -= 1;

                if current.1 == 0 {
                    x += val;
                    pc = None;
                }
                else {
                    pc = Some(current);
                }

            },
        }
    }

    let pick_cycles = vec![20, 60, 100, 140, 180, 220];

    return samples.iter()
        .enumerate()
        .filter_map(|(i, val)| {
            let cycle = (i as i64) + 1;
            if pick_cycles.contains(&cycle) {
                Some(val * cycle)
            }
            else {
                None
            }
        })
        .sum();
}

//fn run_part2(lines: &Vec<&str>) -> usize {
//    return lines.len();
//}
