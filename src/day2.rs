use std::{cmp::Ordering, str::FromStr};

#[derive(PartialEq, Clone, Copy)]
pub enum Shape {
    Rock = 1,
    Paper,
    Scissors,
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let that = *other as u8;
        let this = *self as u8;

        Some(if this == that {
            Ordering::Equal
        } else if this == (that - 1) % 3 {
            Ordering::Less
        } else {
            Ordering::Greater
        })
    }
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => unreachable!(),
        }
    }
}

pub fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|game| {
            let moves = game
                .split(" ")
                .map(|shape| shape.parse::<Shape>().unwrap())
                .collect::<Vec<Shape>>();

            (match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Less) => 6,    /* won! */
                Some(Ordering::Equal) => 3,   /* drew */
                Some(Ordering::Greater) => 0, /* lost */
                None => unreachable!(),
            }) + moves[1] as u32
        })
        .sum::<u32>()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|game| {
            let moves = game.split(" ").collect::<Vec<_>>();
            let opponent = moves[0].parse::<Shape>().unwrap();

            (match moves[1] {
                "X" => {
                    (match opponent {
                        Shape::Rock => Shape::Scissors,
                        Shape::Paper => Shape::Rock,
                        Shape::Scissors => Shape::Paper,
                    }) as u32
                }
                "Y" => 3 + opponent as u32,
                "Z" => {
                    6 + (match opponent {
                        Shape::Rock => Shape::Paper,
                        Shape::Paper => Shape::Scissors,
                        Shape::Scissors => Shape::Rock,
                    }) as u32
                }
                _ => unreachable!(),
            }) as u32
        })
        .sum::<u32>()
        .to_string()
}
