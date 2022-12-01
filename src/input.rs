use std::fs;

pub fn get_input(day: u8) -> String {
    fs::read_to_string(format!("../tests/day{}.txt", day))
        .expect("Input for this day not found.")
}
