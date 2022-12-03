fn priority(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - 96,
        'A'..='Z' => c as u8 - 64 + 26,
        _ => unreachable!(),
    }
}

pub fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|rucksack| {
            let len = rucksack.len();
            let rucksack = rucksack.chars().map(priority).collect::<Vec<_>>();
            let (left, right) = rucksack.split_at(len / 2);

            *left
                .into_iter()
                .find(|&item_t| right.contains(item_t))
                .unwrap() as u64
        })
        .sum::<u64>()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    "".to_owned()
}
