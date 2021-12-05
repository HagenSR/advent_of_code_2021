use std::fs::File;
use std::io::{BufRead, BufReader};
mod heat_map;
use crate::heat_map::HeatMap;

fn main() {
    first();
    second();
}

/// Find tiles where lines overlap at least twice, not considering diagnols
fn first() {
    let filename = "src/data.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut map: HeatMap = HeatMap::new();
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let str = line.unwrap();
        let mut parts = str.split("->");
        let begin: Vec<i32> = parts
            .next()
            .unwrap()
            .split(",")
            .map(|res| res.trim().parse::<i32>().unwrap())
            .collect();
        let end: Vec<i32> = parts
            .next()
            .unwrap()
            .split(",")
            .map(|res| res.trim().parse::<i32>().unwrap())
            .collect();

        if begin[0] == end[0] || begin[1] == end[1] {
            map.add_line((begin[0], begin[1]), (end[0], end[1]));
        }
    }
    println!("{}", map.get_score())
}

// Find tiles where lines collide atleast twice, considering diagnols
fn second() {
    let filename = "src/data.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut map: HeatMap = HeatMap::new();
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let str = line.unwrap();
        let mut parts = str.split("->");
        let begin: Vec<i32> = parts
            .next()
            .unwrap()
            .split(",")
            .map(|res| res.trim().parse::<i32>().unwrap())
            .collect();
        let end: Vec<i32> = parts
            .next()
            .unwrap()
            .split(",")
            .map(|res| res.trim().parse::<i32>().unwrap())
            .collect();
        map.add_line((begin[0], begin[1]), (end[0], end[1]));
    }
    println!("{}", map.get_score())
}
