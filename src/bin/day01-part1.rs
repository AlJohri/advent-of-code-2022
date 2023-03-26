use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/day01.txt";
    let file = File::open(filepath).unwrap();

    let mut current = 0;
    let mut max = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line == "" {
            max = if current > max { current } else { max };

            current = 0;
        } else {
            let number: i32 = line.parse().unwrap();
            current += number;
        }
    }

    // in case the file does not end with an empty line,
    // we need to check if the last collection is larger than the max
    max = if current > max { current } else { max };

    println!("{}", max);

    Ok(())
}
