// Elf calories

pub fn part_1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|elf| -> u64 {
            elf.lines()
                .map(|x| x.trim().parse::<u64>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    let mut res = input
        .split("\n\n")
        .map(|elf| -> u64 {
            elf.lines()
                .filter(|&x| x != "")
                .map(|x| x.trim().parse::<u64>().unwrap())
                .sum()
        })
        .collect::<Vec<_>>();
    res.sort_unstable_by(|a, b| b.cmp(a));

    res.iter().take(3).sum::<u64>().to_string()
}
