use itertools::Itertools;
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

fn find_common_character(group: Vec<Vec<char>>) -> char {
    let mut sets: Vec<HashSet<char>> = group
        .into_iter()
        .map(|x| HashSet::<char>::from_iter(x))
        .collect();
    let (intersection, remaining) = sets.split_first_mut().unwrap();
    for set in remaining {
        intersection.retain(|e| set.contains(e));
    }

    assert!(intersection.len() == 1);

    intersection.iter().next().unwrap().clone()
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/day03.txt";
    let file = File::open(filepath).unwrap();

    let priorities = PriorityMapping::new();

    let mut total = 0;

    let groups = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| line != "")
        .chunks(3);

    for group in &groups {
        let group_of_chars: Vec<Vec<char>> = group
            .map(|line| {
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
                chars
            })
            .collect();

        let common = find_common_character(group_of_chars);
        dbg!(&common);
        let priority = priorities.map.get(&common).unwrap();
        total += priority;
    }

    println!("Total Priority: {total}");

    Ok(())
}
