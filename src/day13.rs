#[derive(Debug)]
enum Element {
    I(u32),
    List(Vec<Element>),
}

type Pair = (Element, Element);

fn parse_pair(_pair: (&str, &str)) -> Pair {
    return (Element::I(0), Element::I(0));
}

pub fn main() {
    println!("[Day13] Solutions:");

    let pairs = include_str!("../data/day13.example")
        .split("\r\n\r\n")
        .map(|s| { parse_pair(s.split_once("\r\n").unwrap()) })
        .collect::<Vec<Pair>>();

    println!("[Day13] Part 1 => {}", run_part1(&pairs));
    //println!("[Day13] Part 2 => {}", run_part2());

    println!("[Day13] Complete -----------------------");
}

fn run_part1(pairs: &Vec<Pair>) -> usize {
    pairs.iter()
        .for_each(|p| println!("{:?}", p));

    return 0;
}

//fn run_part2() -> usize {
//    return 0;
//}
