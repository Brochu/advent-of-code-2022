#[derive(Debug, Clone)]
enum Op {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug, Clone)]
enum Test {
    DivideBy(u64),
}

#[derive(Debug, Clone)]
struct Monkey{
    items: Vec<u64>,
    op: Op,
    test: Test,
    targets: (usize, usize), // (Target idx if true, Target idx if false)
}

fn parse_monkey(monkey: &str) -> Monkey {
    let mut items: Vec<u64> = Vec::new();
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
                        .map(|s| s.trim().parse::<u64>().unwrap()).collect();
                    items.reverse();
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
                    };
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

    let monkeys = parse_monkeys(include_str!("../data/day11.input"));
    let mut p1 = monkeys.clone();
    let mut p2 = monkeys.clone();

    println!("[Day11] Part 1 => {}", run_part1(&mut p1));
    println!("[Day11] Part 2 => {}", run_part2(&mut p2));

    println!("[Day11] Complete -----------------------");
}

fn run_part1(monkeys: &mut Vec<Monkey>) -> u64 {
    let mut check_counts = Vec::<u64>::new();
    check_counts.resize(monkeys.len(), 0);

    (1..=20).for_each(|_round_idx| {

        (0..monkeys.len()).for_each(|mi| {

            (0..monkeys[mi].items.len()).for_each(|_| {
                let m = &mut monkeys[mi];
                let mut item = m.items.pop().unwrap();
                check_counts[mi] += 1;

                match m.op {
                    Op::Add(val) => { item += val },
                    Op::Multiply(val) => { item *= val },
                    Op::Square => { item *= item},
                };

                item /= 3;

                let test_res = match m.test {
                    Test::DivideBy(val) => item % val == 0,
                };

                let (true_idx, false_idx) = m.targets;
                if test_res {
                    monkeys[true_idx].items.push(item);
                }
                else {
                    monkeys[false_idx].items.push(item);
                }
            });
        })
    });

    //monkeys.iter()
    //    .for_each(|m| println!("{:?}\n{:?}\n{:?}\n{:?}\n", m.items, m.op, m.test, m.targets));

    //check_counts.iter()
    //    .enumerate()
    //    .for_each(|(i, c)| println!("Monkey {i} checked {c} items"));

    check_counts.sort();
    check_counts.reverse();

    let monkey_business: u64 = check_counts.iter().take(2).product();
    //println!("Monkey Business: {monkey_business}");

    return monkey_business;
}

fn div_test(mut value: u64, div_val: u64, lcm: u64) -> (bool, u64) {
    value = value % lcm;
    return (value % div_val == 0, value);
}

fn run_part2(monkeys: &mut Vec<Monkey>) -> u64 {
    //monkeys.iter()
    //    .for_each(|m| println!("{:?}\n{:?}\n{:?}\n{:?}\n", m.items, m.op, m.test, m.targets));

    let lcm = monkeys.iter()
        .fold(1, |mult, m| {
            let div = match m.test {
                Test::DivideBy(val) => val,
            };
            mult * div
        });
    //println!("LCM: {lcm}");

    let mut check_counts = Vec::<u64>::new();
    check_counts.resize(monkeys.len(), 0);

    (1..=10_000).for_each(|_round_idx| {
        //println!("Round {_round_idx}");

        (0..monkeys.len()).for_each(|mi| {
            //println!("Monkey {mi}");
            //let m = &monkeys[mi];
            //println!("{:?}\n{:?}\n{:?}\n{:?}\n", m.items, m.op, m.test, m.targets);

            (0..monkeys[mi].items.len()).for_each(|_| {
                let m = &mut monkeys[mi];
                let mut item = m.items.pop().unwrap();
                check_counts[mi] += 1;

                let test_val = match m.test {
                    Test::DivideBy(val) => val,
                };

                let test_res = match m.op {
                    Op::Add(val) => {
                        let (res, new_val) = div_test(item + val, test_val, lcm);
                        item = new_val;
                        res
                    },
                    Op::Multiply(val) => {
                        let (res, new_val) = div_test(item * val, test_val, lcm);
                        item = new_val;
                        res
                    },
                    Op::Square => {
                        let (res, new_val) = div_test(item * item, test_val, lcm);
                        item = new_val;
                        res
                    },
                };

                let (true_idx, false_idx) = m.targets;
                if test_res {
                    monkeys[true_idx].items.push(item);
                }
                else {
                    monkeys[false_idx].items.push(item);
                }
            });
        })
    });

    //check_counts.iter()
    //    .enumerate()
    //    .for_each(|(i, c)| println!("Monkey {i} checked {c} items"));

    check_counts.sort();
    check_counts.reverse();

    let monkey_business: u64 = check_counts.iter().take(2).product();
    //println!("Monkey Business: {monkey_business}");

    return monkey_business;
}
