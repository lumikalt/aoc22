use aoc22::{input::get_input, *};

fn main() {
    dbg!(day4::part_1(&get_input(4)));
    dbg!(day4::part_2(&get_input(4)));
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

        assert_eq!(day1::part_1(INPUT), 24000.to_string());
    }

    #[test]
    fn day_2() {
        const INPUT: &str = "A Y
B X
C Z";

        assert_eq!(day2::part_1(INPUT), 15.to_string());
        assert_eq!(day2::part_2(INPUT), 12.to_string());
    }

    #[test]
    fn day_3() {
        const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(day3::part_1(INPUT), 157.to_string());
        assert_eq!(day3::part_2(INPUT), 70.to_string())
    }

    #[test]
    fn day_4() {
        const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(day4::part_1(INPUT), 2.to_string());
    }
}
