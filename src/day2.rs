pub fn main() {
    println!("[Day2] Solutions:");

    let part1 = run_part1();
    println!("[Day2] Part 1 => {}", part1);

    let part2 = run_part2();
    println!("[Day2] Part 2 => {}", part2);

    println!("[Day2] Complete -----------------------");
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn parse_shapes(s: (&str, &str)) -> (Shape, Shape) {
    let other = match s.0 {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("[Day2] Could not parse shape")
    };

    let yours = match s.1.trim() {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("[Day2] Could not parse shape")
    };

    return (other, yours);
}

fn find_lose(s: &Shape) -> Shape {
    return match s {
        Shape::Rock => Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    };
}

fn find_tie(s: &Shape) -> Shape {
    return match s {
        Shape::Rock => Shape::Rock,
        Shape::Paper => Shape::Paper,
        Shape::Scissors => Shape::Scissors,
    };
}

fn find_win(s: &Shape) -> Shape {
    return match s {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissors,
        Shape::Scissors => Shape::Rock,
    };
}

fn parse_shapes_part2(s: (&str, &str)) -> (Shape, Shape) {
    let other = match s.0 {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("[Day2] Could not parse shape")
    };

    let yours = match s.1.trim() {
        "X" => find_lose(&other),
        "Y" => find_tie(&other),
        "Z" => find_win(&other),
        _ => panic!("[Day2] Could not parse shape")
    };

    return (other, yours);
}

fn calc_outcome(other: Shape, yours: Shape) -> u64 {
    let (base, win) = match yours {
        Shape::Rock => {
            let score = match other {
                Shape::Rock => 3,
                Shape::Paper => 0,
                Shape::Scissors => 6,
            };
            (1, score)
        },
        Shape::Paper => {
            let score = match other {
                Shape::Rock => 6,
                Shape::Paper => 3,
                Shape::Scissors => 0,
            };
            (2, score)
        },
        Shape::Scissors => {
            let score = match other {
                Shape::Rock => 0,
                Shape::Paper => 6,
                Shape::Scissors => 3,
            };
            (3, score)
        },
    };

    return base + win;
}

fn run_part1() -> u64 {
    return include_str!("../data/day2.input")
        .lines()
        .map(|s| parse_shapes(s.split_once(" ").unwrap()))
        .map(|m| calc_outcome(m.0, m.1))
        .collect::<Vec<u64>>()
        .iter()
        .sum();
}

fn run_part2() -> u64 {
    return include_str!("../data/day2.input")
        .lines()
        .map(|s| parse_shapes_part2(s.split_once(" ").unwrap()))
        .map(|m| calc_outcome(m.0, m.1))
        .collect::<Vec<u64>>()
        .iter()
        .sum();
}
