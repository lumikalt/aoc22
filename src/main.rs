use aoc22::{input::get_input, *};

fn main() {
    println!("{}", day2::part_1(&get_input(2)));
    println!("{}", day2::part_2(&get_input(2)));
}

#[cfg(test)]
mod tests {
    use aoc22::day2;

    use super::*;

    #[test]
    fn day_1() {
        const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(day1::part_1(INPUT), 24000u64.to_string());
    }

    #[test]
    fn day_2() {
        const INPUT: &str = "A Y
B X
C Z";

        assert_eq!(day2::part_1(INPUT), 15u64.to_string());
        assert_eq!(day2::part_2(INPUT), 12u64.to_string());
    }
}
