// Elf calories

pub fn part_1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|elf| -> u64 {
            elf.split("\n")
                .map(|x| x.trim().parse::<u64>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn part_2() {}
