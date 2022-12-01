use aoc22::{day1, input::get_input};

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_1() {
        let input = "1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";

        assert_eq!(day1::part_1(input), 24000u64.to_string());
    }
}
