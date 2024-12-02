
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::zip;
use std::ops::Range;

fn file(filename: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn col(lines: &[String], range: Range<usize>) -> Vec<i32> {
    lines.iter().map(|l| l[range.start..range.end].parse::<i32>().unwrap()).sorted().collect()
}

fn step1(col1: &[i32], col2: &[i32]) -> i32 {
    zip(col1, col2).map(|(a, b)| i32::abs(a - b)).sum()
}

fn step2(col1: &[i32], col2: &[i32]) -> i32 {
    let mut map = HashMap::new();
    col2.iter().for_each(|&n| *map.entry(n).or_insert(0) += 1);
    col1.iter().map(|&n| n * *map.entry(n).or_insert(0)).sum()
}

fn main() {
    let lines = file("../input.txt").expect("Could not read file");
    let col1 = col(&lines, 0..5);
    let col2 = col(&lines, 8..13);

    println!("step1: {}", step1(&col1, &col2)); // 1873376
    println!("step2: {}", step2(&col1, &col2)); // 18997088
}

