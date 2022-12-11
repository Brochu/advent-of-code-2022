#[derive(Debug)]
#[derive(Clone)]
enum Op {
    Add(i32),
    Multiply(i32),
    Square,
}

#[derive(Debug)]
#[derive(Clone)]
enum Test {
    DivideBy(i32),
}

#[derive(Debug)]
#[derive(Clone)]
struct Monkey{
    items: Vec<usize>,
    op: Op,
    test: Test,
    targets: (usize, usize), // (Target idx if true, Target idx if false)
}

fn parse_monkey(monkey: &str) -> Monkey {
    let mut items: Vec<usize> = Vec::new();
    let mut op: Op = Op::Square;
    let mut test: Test = Test::DivideBy(0); // Makes sure we crash if we can't parse right
    let mut targets: (usize, usize) = (0, 0);

    monkey.lines()
        .skip(1)
        .for_each(|line| {
            let (name, prop) = line.split_once(":").unwrap();

            match name.trim() {
                "Starting items" => {
                    items = prop.split(",")
                        .map(|s| s.trim().parse::<usize>().unwrap()).collect()
                },
                "Operation" => {
                    let (_, full_op) = prop.split_once("=").unwrap();
                    let (_, op_arg) = full_op.trim().split_once(" ").unwrap();
                    let (operation, arg) = op_arg.split_once(" ").unwrap();

                    match operation {
                        "+" => op = Op::Add(arg.parse().unwrap()),
                        "*" => {
                            op = match arg {
                                "old" => Op::Square,
                                _ => Op::Multiply(arg.parse().unwrap()),
                            };
                        },
                        _ => panic!("[Day11] Invalid operation"),
                    }
                },
                "Test" => {
                    let (_, val) = prop.split_once("by ").unwrap();
                    test = Test::DivideBy(val.parse().unwrap());
                },
                "If true" => {
                    let (_, idx) = prop.split_once("monkey ").unwrap();
                    targets.0 = idx.parse().unwrap();
                },
                "If false" => {
                    let (_, idx) = prop.split_once("monkey ").unwrap();
                    targets.1 = idx.parse().unwrap();
                },
                _ => panic!("[Day11] Could not parse property name : {}", name),
            }
        });

    return Monkey { items, op, test, targets };
}

fn parse_monkeys(data: &str) -> Vec<Monkey> {
    let out = data.split("\r\n\r\n")
        .map(|monkey| parse_monkey(monkey))
        .collect::<Vec<Monkey>>();

    return out;
}

pub fn main() {
    println!("[Day11] Solutions:");

    let monkeys = parse_monkeys(include_str!("../data/day11.example"));
    let mut p1 = monkeys.clone();
    let mut p2 = monkeys.clone();

    println!("[Day11] Part 1 => {}", run_part1(&mut p1));
    println!("[Day11] Part 2 => {}", run_part2(&mut p2));

    println!("[Day11] Complete -----------------------");
}

fn run_part1(monkeys: &mut Vec<Monkey>) -> usize {
    (1..=1).for_each(|round_idx| {
        println!("Round {round_idx}");

        (0..1).for_each(|mi| {
            println!("    Monkey {mi}");

            (0..monkeys[mi].items.len()).for_each(|_| {
                let item = monkeys[mi].items.pop().unwrap();
                println!("        Checking item {item}");
                //TODO: Deal with round steps here
            });
        })
    });

    monkeys.iter()
        .for_each(|m| println!("{:?}\n{:?}\n{:?}\n{:?}\n", m.items, m.op, m.test, m.targets));

    return monkeys.len();
}

fn run_part2(monkeys: &mut Vec<Monkey>) -> usize {
    return monkeys.len();
}
