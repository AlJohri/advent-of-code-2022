use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/day06.txt";
    let file = File::open(filepath).unwrap();

    let mut start: usize = 0;
    let mut map = HashMap::<char, usize>::new();

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line == "" {
            continue;
        }
        let chars: Vec<char> = line.chars().into_iter().collect();

        for (i, char) in chars.iter().enumerate() {
            println!(
                "i: {i}, start: {start}, map: {map:?}, \t\t chars: {:?}",
                &chars[start..i]
            );

            if map.len() == 4 {
                println!();
                println!("done! we have found the start-of-packet marker at indexes {start} to {i} with characters: {:?}", &chars[start..i]);
                break;
            }

            match map.get(&char) {
                Some(duplicate_index) => {
                    let duplicate_index = duplicate_index.clone();
                    println!("duplicate found: {char}");
                    println!(
                        "removing characters from the map from index {start} to {duplicate_index}"
                    );
                    for j in start..duplicate_index {
                        let c = chars[j];
                        println!("removing index {j} character {c}");
                        map.remove(&c);
                    }
                    start = duplicate_index + 1;
                    map.insert(*char, i);
                }
                None => {
                    map.insert(*char, i);
                }
            }
        }
    }

    Ok(())
}
