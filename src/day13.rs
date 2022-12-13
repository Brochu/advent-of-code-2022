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

fn run_part1(_pairs: &Vec<Pair>) -> usize {
    let pairs = include_str!("../data/day13.input")
        .split("\r\n\r\n")
        .map(|s| s.split_once("\r\n").unwrap())
        .collect::<Vec<(&str, &str)>>();

    let res = pairs.iter().map(|&(first, second)| {
        let left_nums = first.chars().filter_map(|c| {
            if c.is_digit(10) {
                Some(c.to_digit(10).unwrap())
            }
            else {
                None
            }
        });
        let right_nums = second.chars().filter_map(|c| {
            if c.is_digit(10) {
                Some(c.to_digit(10).unwrap())
            }
            else {
                None
            }
        });

        let order = left_nums.zip(right_nums)
            .fold(0, |order, (l, r)| {

                if order != 0 {
                    return order;
                }

                //println!("Left = {}; Right = {}", l, r);
                if l == r {
                    return 0;
                }
                else if l < r{
                    return 1;
                }
                else {
                    return -1;
                }
            });

        //println!("Order = {}", order);
        match order {
            -1 => return false,
            0 => return first.len() < second.len(),
            1 => return true,
            _ => panic!("INVALID"),
        }
    })
    .collect::<Vec<bool>>();

    //println!("{:?}", res);
    return res.iter()
        .enumerate()
        .filter_map(|(i, &r)| {
            if r { Some(i+1) } else { None }
        })
        .sum();
}

//fn run_part2() -> usize {
//    return 0;
//}
