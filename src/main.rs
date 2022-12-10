use lib::days::day3;
use lib::read_file;
fn main() {
    let path = "../input/day3.txt";

    let file = read_file(path);
    // println!("{file}");
    // let result = day3::main(file);

    // println!("Result: {result}");

    // dbg!(result);

    let res = test(file);
    println!("RESULT FROM MAIN: {res}"); //THIS WAS THE ANSWER! I guess you lose data when converting larger ints to i32's?????? <-learn more about this
}
fn test(input: String) -> usize {
    let lines = input.lines();
    let mut lookup_table = ('a'..='z').into_iter().collect::<Vec<char>>();
    let rnge = ('A'..='Z').into_iter();
    for lett in rnge {
        lookup_table.push(lett);
    }

    let mut sum: usize = 0;
    for line in lines {
        let duplicate_option = match returns_duplicate(line) {
            Some(c) => c,
            None => panic!("WTF?!?"),
        };

        sum += lookup(duplicate_option, &lookup_table);
    }

    fn returns_duplicate(line: &str) -> Option<char> {
        let mut comp_one: Vec<char> = vec![];
        for (i, ch) in line.chars().enumerate() {
            // println!("Compartment one: {:?}", comp_one);
            // println!("ch: {ch}");
            if i < line.len() / 2 {
                comp_one.push(ch);
            } else if comp_one.contains(&ch) {
                // println!("Ch found: {ch}");
                // return Some(ch);
                return Some(ch);
            }
        }
        None
    }

    fn lookup(ch: char, table: &Vec<char>) -> usize {
        let mut result: usize = 0;
        for (i, c) in table.iter().enumerate() {
            println!("I: {i}    C:{c}");
            if *c == ch {
                result += i + 1;
            }
        }
        return result;
    }

    sum
}
