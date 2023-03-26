use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum MyGameResult {
    Win = 6,
    Loss = 0,
    Tie = 3,
}

#[derive(Debug, Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
#[derive(Debug)]
struct MyShape(Shape);

#[derive(Debug)]
struct OpponentShape(Shape);

impl FromStr for OpponentShape {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Box<(dyn std::error::Error + 'static)>> {
        match s {
            "A" => Ok(Self(Shape::Rock)),
            "B" => Ok(Self(Shape::Paper)),
            "C" => Ok(Self(Shape::Scissors)),
            _ => return Err(format!("invalid opponent shape: {}", s).into()),
        }
    }
}

impl FromStr for MyShape {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Box<(dyn std::error::Error + 'static)>> {
        match s {
            "X" => Ok(Self(Shape::Rock)),
            "Y" => Ok(Self(Shape::Paper)),
            "Z" => Ok(Self(Shape::Scissors)),
            _ => return Err(format!("invalid my shape: {}", s).into()),
        }
    }
}

impl Add<&OpponentShape> for &MyShape {
    type Output = MyGameResult;

    fn add(self, opponent: &OpponentShape) -> Self::Output {
        match (&self.0, &opponent.0) {
            (Shape::Rock, Shape::Rock) => MyGameResult::Tie,
            (Shape::Rock, Shape::Scissors) => MyGameResult::Win,
            (Shape::Rock, Shape::Paper) => MyGameResult::Loss,

            (Shape::Paper, Shape::Paper) => MyGameResult::Tie,
            (Shape::Paper, Shape::Rock) => MyGameResult::Win,
            (Shape::Paper, Shape::Scissors) => MyGameResult::Loss,

            (Shape::Scissors, Shape::Scissors) => MyGameResult::Tie,
            (Shape::Scissors, Shape::Paper) => MyGameResult::Win,
            (Shape::Scissors, Shape::Rock) => MyGameResult::Loss,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/day02.txt";
    let file = File::open(filepath).unwrap();

    let mut total = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line == "" {
            continue;
        }

        let (opponent, me) = match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [opponent, me] => (
                OpponentShape::from_str(opponent).unwrap(),
                MyShape::from_str(me).unwrap(),
            ),
            _ => return Err(format!("invalid line: {}", line).into()),
        };

        let result = &me + &opponent;
        let score = me.0 as i32 + result as i32;

        total += score;

        println!(
            "{:?} + {:?} = {:?} which has a score of {} + {} = {}. Current Total: {}",
            &me, &opponent, &result, me.0 as i32, result as i32, &score, &total
        );
    }

    println!("Final Total: {}", total);

    Ok(())
}
