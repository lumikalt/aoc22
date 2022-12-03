use std::fs;

pub fn get_input(day: u8) -> String {
    fs::read_to_string(format!("./tests/day{}.txt", day))
        .expect(&format!("Input for day {} not found.", day))
        .trim()
        .to_string()
}
