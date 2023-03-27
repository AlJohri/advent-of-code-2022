use core::iter::zip;
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            data: Vec::<T>::new(),
        }
    }

    fn push(&mut self, c: T) {
        self.data.push(c);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

#[derive(Display, Debug)]
struct Crate(char);

/// Ship has multiple stacks of crates.
struct Ship {
    stacks: BTreeMap<u32, Stack<Crate>>,
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ship:\n")?;
        for (num, stack) in &self.stacks {
            write!(f, "{num}: {stack:?}\n")?;
        }
        Ok(())
    }
}

impl TryFrom<Vec<String>> for Ship {
    type Error = Box<dyn Error>;

    fn try_from(lines: Vec<String>) -> Result<Ship, Box<(dyn std::error::Error + 'static)>> {
        let mut ship = Self {
            stacks: BTreeMap::new(),
        };

        let (last, remaining) = lines.split_last().unwrap();
        let column_nums: Vec<u32> = last
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        for line in remaining.iter().rev() {
            let columns = line.chars().chunks(4);

            for (mut col, num) in zip(columns.into_iter(), &column_nums) {
                let character = col.nth(1).unwrap();
                if character == ' ' {
                    continue;
                }
                let krate = Crate(character);
                let stack = ship
                    .stacks
                    .entry(*num)
                    .or_insert_with(|| Stack::<Crate>::new());
                stack.push(krate);
            }
        }

        Ok(ship)
    }
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("move {num} from {from} to {to}")]
struct MoveInstruction {
    num: u32,
    from: u32,
    to: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/day05.txt";

    // collect the initial lines of the file representing the ship (stacks of crates)
    let mut ship_lines = Vec::<String>::new();
    let file = File::open(&filepath).unwrap();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line == "" {
            continue;
        }
        if line.starts_with(" 1") {
            ship_lines.push(line);
            break;
        }
        ship_lines.push(line);
    }

    // create ship
    let mut ship: Ship = ship_lines.try_into().unwrap();
    println!("{ship}");

    // apply move instructions to the ship
    let file = File::open(&filepath).unwrap();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if !line.starts_with("move") {
            continue;
        }
        let instruction: MoveInstruction = line.parse().unwrap();

        // This is the only part that changed for Day 5, Part 2. Instead of
        // popping and pushing each krate one at a time. We pop them all, and
        // then in reverse push them onto the target stack.
        let from_stack = ship.stacks.get_mut(&instruction.from).unwrap();
        let krates: Vec<Crate> = (0..instruction.num)
            .into_iter()
            .map(|_| from_stack.pop().unwrap())
            .collect();
        let to_stack = ship.stacks.get_mut(&instruction.to).unwrap();
        krates.into_iter().rev().for_each(|x| to_stack.push(x));
    }

    println!("{ship}");

    print!("Top of Each Stack from Left to Right: ");
    for (_, stack) in &ship.stacks {
        print!("{}", stack.peek().unwrap());
    }
    println!();

    Ok(())
}
