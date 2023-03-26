use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

// https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html#min-heap
struct SortedTopK<T: Ord> {
    k: usize,
    heap: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> SortedTopK<T> {
    fn new(k: usize) -> Self {
        Self {
            k: k,
            heap: BinaryHeap::new(),
        }
    }

    fn push(&mut self, x: T) {
        self.heap.push(Reverse(x));

        if self.heap.len() > self.k {
            self.heap.pop().unwrap();
        }
    }

    fn get_topk(&mut self) -> Vec<T> {
        let mut output = Vec::<T>::with_capacity(self.k);
        for _ in 1..=self.k {
            output.push(self.heap.pop().unwrap().0);
        }
        output
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/day01.txt";
    let file = File::open(filepath).unwrap();
    let mut topk = SortedTopK::<i32>::new(3 as usize);

    let mut current = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line == "" {
            topk.push(current);
            current = 0;
        } else {
            let number: i32 = line.parse().unwrap();
            current += number;
        }
    }

    println!("{}", topk.get_topk().into_iter().sum::<i32>());

    Ok(())
}
