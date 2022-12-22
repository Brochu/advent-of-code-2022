pub fn main() {
    println!("[Day20] Solutions:");

    let list = include_str!("../data/day20.input")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("[Day20] Part 1 => {}", run_part1(&list));
    //println!("[Day20] Part 2 => {}", run_part2());

    println!("[Day20] Complete -----------------------");
}

fn run_part1(list: &Vec<i32>) -> i32 {
    // indexes into list
    let mut mixed: Vec<_> = (0..list.len()).collect();
    for (idx, &num) in list.iter().enumerate() {
        // find mixed that corresponds to the number in list
        let mixed_idx = mixed.iter().position(|&mix_num| mix_num == idx).unwrap();
        // remove that index from mixed
        mixed.remove(mixed_idx);
        // add num offset to that number and add it back
        let new_mixed_idx = (mixed_idx as i32 + num).rem_euclid(mixed.len() as i32) as usize;
        mixed.insert(new_mixed_idx, idx);
    }

    let zero_idx = list.iter().position(|&num| num == 0).unwrap();
    let zero_mixed_idx = mixed
        .iter()
        .position(|&mix_num| mix_num == zero_idx)
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|offset| {
            let mixed_idx = (zero_mixed_idx + offset) % mixed.len();
            let list_idx = mixed[mixed_idx];
            list[list_idx]
        })
        .sum()
}

//fn run_part2() -> usize {
//    return 0;
//}
