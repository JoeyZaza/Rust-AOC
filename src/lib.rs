// use days::day2::*;

use std::fs::read_to_string;
pub mod days;
pub fn read_file(path: &str) -> String {
    let file = read_to_string(path).unwrap();
    file
}
#[cfg(test)]
mod tests {
    use crate::days::day3;

    #[test]
    fn day3_test_input() {
        let test_input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = day3::main(test_input.to_string());
        assert_eq!(result, 157);
    }
}
