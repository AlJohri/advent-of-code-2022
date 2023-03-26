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

impl FromStr for MyGameResult {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Box<(dyn std::error::Error + 'static)>> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Win),
            _ => return Err(format!("invalid game result: {}", s).into()),
        }
    }
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

fn get_my_shape(o: &OpponentShape, r: &MyGameResult) -> MyShape {
    match (&o.0, &r) {
        (Shape::Rock, MyGameResult::Tie) => MyShape(Shape::Rock),
        (Shape::Paper, MyGameResult::Tie) => MyShape(Shape::Paper),
        (Shape::Scissors, MyGameResult::Tie) => MyShape(Shape::Scissors),

        (Shape::Rock, MyGameResult::Win) => MyShape(Shape::Paper),
        (Shape::Paper, MyGameResult::Win) => MyShape(Shape::Scissors),
        (Shape::Scissors, MyGameResult::Win) => MyShape(Shape::Rock),

        (Shape::Rock, MyGameResult::Loss) => MyShape(Shape::Scissors),
        (Shape::Paper, MyGameResult::Loss) => MyShape(Shape::Rock),
        (Shape::Scissors, MyGameResult::Loss) => MyShape(Shape::Paper),
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

        let (opponent, result) = match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [opponent, result] => (
                OpponentShape::from_str(opponent).unwrap(),
                MyGameResult::from_str(result).unwrap(),
            ),
            _ => return Err(format!("invalid line: {}", line).into()),
        };

        let me = get_my_shape(&opponent, &result);
        let score = me.0 as i32 + result as i32;

        total += score;

        println!("Given {opponent:?} and desired result of {result:?}, my shape must be: {me:?}. This gives a resulting score of {score}. Current Total: {total}");
    }

    println!("Final Total: {}", total);

    Ok(())
}
