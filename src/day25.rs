pub fn main() {
    println!("[Day25] Solutions:");

    let snafu_nums = include_str!("../data/day25.input")
        .lines()
        .collect::<Vec<&str>>();

    println!("[Day25] Part 1 => Number: {}", run_part1(&snafu_nums));

    println!("[Day25] Complete -----------------------");
}

fn snafu_to_dec(snafu: &str) -> i64 {
    let dec = snafu.chars().rev().enumerate()
        .fold(0 as i64, |acc, (i, d)| {
            let val = match d {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("Invalid snafu digit"),
            };
            let pos = (5 as i64).pow(i as u32);

            acc + (val * pos)
        });

    //println!("Converting from (snafu): '{}' to (decimal): {}", snafu, dec);
    return dec;
}

fn dec_to_snafu(mut dec: i64) -> String {
    let mut base5 = Vec::new();
    while dec > 0 {
        base5.push((dec % 5) as u8);
        dec /= 5;
    }

    let (snafu, rest) = base5.iter()
        .fold((String::new(), 0 as u8), |(snafu, rest), &num| {
            //println!("Handling {}, (snafu: {}, rest: {})", num, snafu, rest);
            match num + rest {
                0 => (format!("{}{}", '0', snafu), 0),
                1 => (format!("{}{}", '1', snafu), 0),
                2 => (format!("{}{}", '2', snafu), 0),
                3 => (format!("{}{}", '=', snafu), 1),
                4 => (format!("{}{}", '-', snafu), 1),
                5 => (format!("{}{}", '0', snafu), 1),
                6 => (format!("{}{}", '1', snafu), 1),
                7 => (format!("{}{}", '2', snafu), 1),
                8 => (format!("{}{}", '=', snafu), 2),
                9 => (format!("{}{}", '-', snafu), 2),
                _ => panic!("Invalid digit for base5"),
            }
        });

    if rest > 0 {
        //TODO: Need to handle leftover rest
        panic!("Leftover rest");
    }

    //println!("{}; {}", snafu, rest);
    return snafu;
}

fn run_part1(snafu_nums: &Vec<&str>) -> String {
    let dec_sum: i64 = snafu_nums.iter()
        .map(|n| snafu_to_dec(n))
        .sum();

    //println!();
    //println!("Sum (in decimal): {}", dec_sum);

    return dec_to_snafu(dec_sum);
}
