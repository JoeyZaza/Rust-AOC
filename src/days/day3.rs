use std::vec;

pub fn main(input: String) -> usize {
    let lines = input.lines();
    let mut lookup_table = ('a'..='z').into_iter().collect::<Vec<char>>();
    let rnge = ('A'..='Z').into_iter();
    for lett in rnge {
        lookup_table.push(lett);
    }

    let mut v = vec![];
    let mut sum = 0;
    for line in lines {
        let duplicate_character = returns_duplicate(line);
        let character_score_result = lookup(duplicate_character, &lookup_table);
        let character_score = match character_score_result {
            Err(e) => panic!("Error: {e}"),
            Ok(res) => res,
        };
        // println!("Character score: {character_score}");
        sum += character_score;
        // println!("RESULT: {character_score}");
        v.push(duplicate_character);
    }
    return sum;
}

fn lookup(ch: Option<char>, table: &Vec<char>) -> Result<usize, &'static str> {
    if let Some(_character) = ch {
        for i in 1..table.len() + 1 {
            if ch.unwrap() == table[i - 1] {
                return Ok(i);
            }
        }
    } else {
        return Err("Lookup function received a None type");
    }
    return Ok(0);
}

fn returns_duplicate(line: &str) -> Option<char> {
    let mut comp_one: Vec<char> = vec![];
    for (i, ch) in line.chars().enumerate() {
        if i < line.len() / 2 {
            comp_one.push(ch);
        } else if comp_one.contains(&ch) {
            return Some(ch);
        }
    }

    None
}
