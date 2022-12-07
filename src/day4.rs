use std::ops::RangeInclusive;

use nom::{bytes::streaming::tag, character::complete, sequence::separated_pair, IResult};

fn range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    let (input, (start, end)) = separated_pair(complete::u64, tag("-"), complete::u64)(input)?;

    Ok((input, start..=end))
}

fn parse_line(input: &str) -> IResult<&str, (RangeInclusive<u64>, RangeInclusive<u64>)> {
    let (input, (first, second)) = separated_pair(range, tag(","), range)(input)?;

    Ok((input, (first, second)))
}

pub fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let Ok((_, (first, second))) = parse_line(line) else {unreachable!()};

            (first, second)
        })
        .filter(|(first, second)| {
            first.clone().all(|x| second.contains(&x)) || second.clone().all(|x| first.contains(&x))
        })
        .count()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let Ok((_, (first, second))) = parse_line(line) else {unreachable!()};

            (first, second)
        })
        .filter(|(first, second)| {
            first.clone().any(|x| second.contains(&x)) || second.clone().any(|x| first.contains(&x))
        })
        .count()
        .to_string()
}
