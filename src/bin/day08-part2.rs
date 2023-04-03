use itertools::Itertools;
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, Lines};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn value(self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

fn parse(lines: Lines<io::BufReader<File>>) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = vec![];
    for line in lines {
        let line = line.unwrap();
        if line == "" {
            continue;
        } else {
            let row: Vec<_> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
            grid.push(row);
        }
    }
    grid
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/day08.txt";
    let file = File::open(filepath).unwrap();

    let lines = io::BufReader::new(file).lines();
    let grid = parse(lines);

    let mut max_scenic_score = 0;

    let height = grid.len();
    let width = grid[0].len();

    for i in 0..height {
        for j in 0..width {
            let start = grid[i][j];
            let distances = Direction::iter()
                .map(|dir| {
                    let mut distance = 0;
                    // println!(
                    //     "Starting from {i} {j} with value: {} and going {:?}",
                    //     grid[i][j], dir
                    // );
                    let (add_x, add_y) = dir.value();
                    let (mut x, mut y) = (i.clone(), j.clone());
                    while x > 0 && x < width - 1 && y > 0 && y < height - 1 {
                        x = usize::try_from((x as i32) + add_x).unwrap();
                        y = usize::try_from((y as i32) + add_y).unwrap();
                        let current = grid[x][y];
                        distance += 1;
                        // println!("at {x} {y} with value: {}", grid[x][y]);
                        if current >= start {
                            break;
                        }
                    }
                    distance
                })
                .collect_vec();
            let product = distances.iter().product::<u32>();
            max_scenic_score = max_scenic_score.max(product);
            // println!(
            //     "from {i} {j} with value {start}, we have distances: {:?} with product: {product}. current max: {max_scenic_score}",
            //     distances
            // );
        }
    }

    println!("Max Scenic Score: {max_scenic_score}");

    Ok(())
}
