use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct PriorityMapping {
    map: HashMap<char, i32>,
}

impl PriorityMapping {
    fn new() -> Self {
        let mut map = HashMap::<char, i32>::new();

        // a-z is priority 1-26
        for i in 97..=122 {
            let chr = i as u8 as char;
            let priority = i - 97 + 1;
            map.insert(chr, priority);
        }

        // A-Z is priority 27-52
        for i in 65..=90 {
            let chr = i as u8 as char;
            let priority = i - 65 + 27;
            map.insert(chr, priority);
        }

        PriorityMapping { map }
    }
}

fn find_common_character(first: &[char], second: &[char]) -> Option<char> {
    let mut set = HashSet::new();

    for char in first {
        set.insert(char);
    }

    for char in second {
        if set.contains(char) {
            return Some(char.to_owned());
        }
    }

    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/day03.txt";
    let file = File::open(filepath).unwrap();

    let priorities = PriorityMapping::new();

    let mut total = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line == "" {
            continue;
        }
        let chars: Vec<char> = line.chars().collect();
        if chars.len() % 2 != 0 {
            panic!(
                "{}",
                format!(
                    "uneven number of characters, cannot split line half. line: {}",
                    line
                )
            )
        }
        let middle = chars.len() / 2;
        let first = &chars[..middle];
        let second = &chars[middle..];
        let common = find_common_character(&first, &second).expect("no common character");
        let priority = priorities.map.get(&common).unwrap();
        total += priority;
    }

    println!("Total Priority: {total}");

    Ok(())
}
