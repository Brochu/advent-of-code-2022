use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    Value(i64),
    Add(String, String),
    Sub(String, String),
    Mult(String, String),
    Div(String, String),
}

pub fn main() {
    println!("[Day21] Solutions:");

    let monkeys: HashMap<String, Op> = include_str!("../data/day21.input")
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once(": ").unwrap();
            let mut elems = rest.split(" ");
            let first = String::from(elems.next().unwrap());

            let op = match first.parse::<i64>() {
                Ok(v) => Op::Value(v),
                Err(_) => {
                    let op_str = elems.next().unwrap();
                    let second = String::from(elems.next().unwrap());

                    match op_str {
                        "+" => Op::Add(first, second),
                        "-" => Op::Sub(first, second),
                        "*" => Op::Mult(first, second),
                        "/" => Op::Div(first, second),
                        _ => panic!("Could not parse operation symbol"),
                    }
                }
            };

            (String::from(name), op)
        })
        .collect();

    println!("[Day21] Part 1 => {}", run_part1(&monkeys));
    //println!("[Day21] Part 2 => {}", run_part2());

    println!("[Day21] Complete -----------------------");
}

fn solve(monkeys: &HashMap<String, Op>, op_str: String) ->i64 {
    let op = &monkeys[&op_str];

    match op {
        Op::Value(v) => *v,
        Op::Add(l, r) => solve(monkeys, l.to_string()) + solve(monkeys, r.to_string()),
        Op::Sub(l, r) => solve(monkeys, l.to_string()) - solve(monkeys, r.to_string()),
        Op::Mult(l, r) => solve(monkeys, l.to_string()) * solve(monkeys, r.to_string()),
        Op::Div(l, r) => solve(monkeys, l.to_string()) / solve(monkeys, r.to_string()),
    }
}

fn run_part1(monkeys: &HashMap<String, Op>) -> i64 {
    //monkeys.iter()
    //    .for_each(|m| println!("{:?}", m));

    return solve(&monkeys, String::from("root"));
}

//fn run_part2() -> usize {
//    return 0;
//}
