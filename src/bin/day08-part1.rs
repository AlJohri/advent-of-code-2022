use itertools::{Either, Itertools};
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug)]
enum Axis {
    Row,
    Column,
}

impl Direction {
    fn get_range(
        &self,
        size: i32,
    ) -> (
        impl Iterator<Item = usize> + Debug + Clone,
        impl Iterator<Item = usize> + Debug + Clone,
        Axis,
    ) {
        let last: usize = (size - 1).try_into().unwrap();

        let forward = Either::Left(0..=last);
        let reverse: Either<RangeInclusive<usize>, std::iter::Rev<RangeInclusive<usize>>> =
            Either::Right((0..=last).rev());

        match *self {
            Direction::Right => (forward.clone(), forward.clone(), Axis::Row),
            Direction::Down => (forward.clone(), forward.clone(), Axis::Column),
            Direction::Left => (reverse.clone(), reverse.clone(), Axis::Row),
            Direction::Up => (reverse.clone(), reverse.clone(), Axis::Column),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Cell<T> {
    value: T,
    pos: (usize, usize),
}

pub struct Grid<T>(Vec<Vec<Cell<T>>>);

impl<T: Clone + Copy + Debug> Grid<T> {
    pub fn new() -> Self {
        Grid(vec![])
    }

    pub fn new_with_size(value: T, size: &usize) -> Self {
        Grid(
            (0..*size)
                .map(|i| {
                    (0..*size)
                        .map(|j| Cell { value, pos: (i, j) })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl fmt::Display for Grid<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for cell in row {
                write!(f, "{}", cell.value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Grid<bool> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for cell in row {
                write!(f, "{}", if cell.value { "T" } else { "F" })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<io::Lines<io::BufReader<File>>> for Grid<u32> {
    fn from(lines: io::Lines<io::BufReader<File>>) -> Self {
        let mut grid = Grid::<u32>::new();
        for (i, line) in lines.enumerate() {
            let line = line.unwrap();
            if line == "" {
                continue;
            } else {
                let row: Vec<_> = line
                    .chars()
                    .enumerate()
                    .map(|(j, x)| Cell {
                        value: x.to_digit(10).unwrap(),
                        pos: (i, j),
                    })
                    .collect();
                grid.0.push(row);
            }
        }
        grid
    }
}

impl<T: std::fmt::Debug + Clone> Grid<T> {
    fn iterate_in_direction(&self, direction: Direction) -> Vec<Vec<Cell<T>>> {
        let (x_range, y_range, axis) = direction.get_range(self.0.len().try_into().unwrap());

        match axis {
            Axis::Row => x_range
                .map(|i| {
                    y_range
                        .clone()
                        .map(|j| self.0[i][j].clone())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>(),
            Axis::Column => y_range
                .map(|j| {
                    x_range
                        .clone()
                        .map(|i| self.0[i][j].clone())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/day08.txt";
    let file = File::open(filepath).unwrap();

    let grid: Grid<u32> = io::BufReader::new(file).lines().into();
    println!("{}", &grid);

    let size = grid.0.len();

    println!("Initialize Visibility to All False");
    let mut visibility: Grid<bool> = Grid::new_with_size(false, &size);
    println!("{}", &visibility);

    println!("Set All Border to Visible");
    (0..size).cartesian_product(0..size).for_each(|(i, j)| {
        if i == 0 || j == 0 || i == size - 1 || j == size - 1 {
            visibility.0[i][j].value = true;
        }
    });
    println!("{}", &visibility);

    Direction::iter().for_each(|direction| {
        println!("Process {direction:?}");
        for (k, mut axis) in grid.iterate_in_direction(direction).into_iter().enumerate() {
            if k == 0 {
                println!("skipping first axis");
                continue;
            }
            let (first, remaining) = axis.split_first_mut().unwrap();
            let mut previous_height = first.value;
            println!("initializing previous height to {previous_height} from the first value in axis at position: {:?}", first.pos);
            for cell in remaining {
                println!(
                    "comparing cell.value ({}) to previous_height ({}) at position {:?}",
                    cell.value, previous_height, cell.pos
                );
                if cell.value > previous_height {
                    println!("cell.value ({}) was larger than previous_height ({}). setting cell's visibility at position of {:?} to true", cell.value, previous_height, cell.pos);
                    let (i, j) = cell.pos;
                    visibility.0[i][j].value = true;
                    previous_height = cell.value;
                }
            }
        }
        println!("{}", &visibility);
    });

    let num_visible: u32 = visibility
        .0
        .into_iter()
        .flat_map(|x| x)
        .map(|x| x.value)
        .filter(|&x| x)
        .map(|x| x as u32)
        .sum();

    println!("{num_visible}");

    Ok(())
}
