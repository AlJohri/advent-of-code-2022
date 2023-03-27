use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/day04.txt";
    let file = File::open(filepath).unwrap();

    let mut count = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line == "" {
            continue;
        }
        let split: Vec<&str> = line.split(',').collect();
        assert!(split.len() == 2);

        let first = split[0]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let second = split[1]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let (f_start, f_end) = (first[0], first[1]);
        let (s_start, s_end) = (second[0], second[1]);

        // Condition 1:
        //       f_start ... f_end
        // [ s_start .......... s_end ]
        let cond1 = f_start >= s_start && f_end <= s_end;

        // Condition 2:
        //       s_start ... s_end
        // [ f_start .......... f_end ]
        let cond2 = f_start <= s_start && f_end >= s_end;

        if cond1 || cond2 {
            count += 1;
        }
    }

    println!("Number of Fully Contained Pairs: {count}");

    Ok(())
}
