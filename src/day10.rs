#[derive(Debug)]
enum Op {
    Exit,
    Noop,
    Addx(i64),
}

fn num_cycles(op: &Op) -> usize {
    match op {
        Op::Exit => 1,
        Op::Noop => 1,
        Op::Addx(_) => 2,
    }
}

fn run_program(program: &Vec<Op>) -> Vec<i64> {
    let mut prog_iter = program.iter();
    let mut pc: Option<(&Op, usize)> = None;
    let mut x: i64 = 1;
    
    let mut samples: Vec<i64> = Vec::new();

    loop {
        let (op, clocks) = pc.unwrap_or_else(|| {
            match prog_iter.next() {
                Some(op) => (op, num_cycles(op)),
                None => (&Op::Exit, num_cycles(&Op::Exit)),
            }
        });

        samples.push(x);

        if clocks == 1 {

            match op {
                Op::Exit => break,
                Op::Noop => { },
                Op::Addx(val) => { x += val },
            };

            pc = None;
        }
        else {
            pc = Some((op, clocks - 1));
        }
    };

    return samples;
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

    let samples = run_program(&program);

    println!("[Day10] Part 1 => {}", run_part1(&samples));
    println!("[Day10] Part 2 => ");
    run_part2(&samples).iter()
        .for_each(|pix| println!("{}", pix));

    println!("[Day10] Complete -----------------------");
}

fn run_part1(samples: &Vec<i64>) -> i64 {
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

fn run_part2(samples: &Vec<i64>) -> Vec<String> {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;

    let output = samples[0..(WIDTH * HEIGHT)]
        .chunks(WIDTH)
        .map(|w| {
            w.iter().enumerate().map(|(i, x)| {
                if (i as i64 - x).abs() <= 1 { "#" } else { "." }
            })
            .collect::<String>()
        })
        .collect::<Vec<String>>();

    return output;
}
