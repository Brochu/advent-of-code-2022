pub fn main() {
    println!("[DAY1] Starting day 1");
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one() {
        let size = include_str!("../data/day1.example")
            .bytes()
            .count();
        assert_ne!(size, 0)
    }

    #[test]
    fn part_two() {
        let size = include_str!("../data/day1.example")
            .bytes()
            .count();
        assert_ne!(size, 0)
    }
}
