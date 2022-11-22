pub fn init() {
    println!("[DAY1] Starting day 1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2022::read_file;

    #[test]
    fn part_one() {
        init();

        let content = read_file("example", 1);
        assert_ne!(content.len(), 0)
    }

    #[test]
    fn part_two() {
        init();

        let content = read_file("example", 1);
        assert_ne!(content.len(), 0)
    }
}
